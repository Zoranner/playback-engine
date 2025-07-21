//! 数据集写入器模块
//!
//! 提供高级的数据集写入功能，支持多文件自动切换、索引生成等功能。

use chrono::Datelike;
use log::{debug, info, warn};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::business::config::Configuration;
use crate::foundation::error::{PcapError, Result};
use crate::data::file_writer::PcapFileWriter;
use crate::data::models::{DataPacket, DatasetInfo, FileInfo};
use crate::foundation::traits::{Info, Write};

// 错误消息常量
const ERR_WRITER_FINALIZED: &str = "写入器已完成，无法继续写入";
const ERR_CORRUPTED_DATA: &str = "数据包校验和不匹配";
const ERR_WRITER_NOT_INIT: &str = "文件写入器未初始化";

/// 数据集写入器
///
/// 提供对整个PCAP数据集的统一写入接口，支持多文件自动切换、索引生成等功能。
pub struct Writer {
    /// 基础路径
    base_path: PathBuf,
    /// 数据集名称
    dataset_name: String,
    /// 配置信息（使用 Arc 避免克隆）
    configuration: Arc<Configuration>,
    /// 当前文件写入器
    current_writer: Option<PcapFileWriter>,
    /// 当前文件路径
    current_file_path: Option<PathBuf>,
    /// 当前文件中的数据包计数
    current_file_packet_count: usize,
    /// 文件序号
    file_sequence: usize,
    /// 数据集信息
    dataset_info: DatasetInfo,
    /// 总数据包计数
    total_packet_count: u64,
    /// 第一个数据包时间戳
    first_timestamp: Option<u64>,
    /// 最后一个数据包时间戳
    last_timestamp: Option<u64>,
    /// 创建的文件列表
    created_files: Vec<PathBuf>,
    /// 总大小缓存（使用RefCell实现内部可变性）
    total_size_cache: RefCell<Option<u64>>,
    /// 统计信息是否需要更新
    statistics_dirty: bool,
    /// 是否已完成
    is_finalized: bool,
}

impl Writer {
    /// 创建新的数据集写入器
    ///
    /// # 参数
    /// - `base_path` - 基础目录路径
    /// - `dataset_name` - 数据集名称
    /// - `config` - PCAP配置
    ///
    /// # 返回
    /// 返回初始化后的写入器实例
    pub fn new<P: AsRef<Path>>(
        base_path: P,
        dataset_name: &str,
        config: Configuration,
    ) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        let dataset_path = base_path.join(dataset_name);

        // 创建数据集目录（如果不存在）
        if !dataset_path.exists() {
            std::fs::create_dir_all(&dataset_path).map_err(|e| PcapError::Io(e))?;
            info!("已创建数据集目录: {:?}", dataset_path);
        } else {
            // 检查目录是否为空或只包含隐藏文件
            let entries: Vec<_> = std::fs::read_dir(&dataset_path)
                .map_err(|e| PcapError::Io(e))?
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    // 过滤掉隐藏文件和系统文件
                    if let Some(name) = entry.file_name().to_str() {
                        !name.starts_with('.') && !name.starts_with('~')
                    } else {
                        false
                    }
                })
                .collect();

            if !entries.is_empty() {
                return Err(PcapError::InvalidState(format!(
                    "数据集目录已存在且包含文件: {:?}",
                    dataset_path
                )));
            }
        }

        if !dataset_path.is_dir() {
            return Err(PcapError::InvalidArgument(format!(
                "指定路径不是目录: {:?}",
                dataset_path
            )));
        }

        // 初始化数据集信息
        let dataset_info = DatasetInfo::new(dataset_name.to_string(), &dataset_path);

        // 预分配文件列表容量（基于预估数据包数量）
        let estimated_files = if config.max_packets_per_file > 0 {
            (config.max_packets_per_file.saturating_sub(1) / config.max_packets_per_file) + 1
        } else {
            10 // 默认预分配10个文件
        };

        info!("数据集写入器已初始化: {:?}", dataset_path);

        Ok(Self {
            base_path,
            dataset_name: dataset_name.to_string(),
            configuration: Arc::new(config),
            current_writer: None,
            current_file_path: None,
            current_file_packet_count: 0,
            file_sequence: 0,
            dataset_info,
            total_packet_count: 0,
            first_timestamp: None,
            last_timestamp: None,
            created_files: Vec::with_capacity(estimated_files),
            total_size_cache: RefCell::new(None),
            statistics_dirty: false,
            is_finalized: false,
        })
    }

    /// 获取数据集完整路径
    fn dataset_path(&self) -> PathBuf {
        self.base_path.join(&self.dataset_name)
    }

    /// 生成下一个文件名
    fn generate_next_filename(&self) -> String {
        use chrono::{Timelike, Utc};

        let now = Utc::now();
        let filename = match self.configuration.file_name_format.as_str() {
            "yyMMdd_HHmmss_fffffff" => {
                format!(
                    "{:02}{:02}{:02}_{:02}{:02}{:02}_{:07}.pcap",
                    now.year() % 100,
                    now.month(),
                    now.day(),
                    now.hour(),
                    now.minute(),
                    now.second(),
                    now.nanosecond() / 100 // 转换为百纳秒
                )
            }
            _ => {
                // 默认格式
                format!("data_{:04}.pcap", self.file_sequence)
            }
        };

        filename
    }

    /// 创建新的文件写入器
    fn create_new_file(&mut self) -> Result<()> {
        // 关闭当前文件
        if let Some(ref mut writer) = self.current_writer {
            writer.close();
        }

        // 生成新文件名
        let filename = self.generate_next_filename();
        let file_path = self.dataset_path().join(&filename);

        // 创建新的文件写入器
        let mut writer = PcapFileWriter::new((*self.configuration).clone());
        writer.create(&file_path)?;

        self.current_writer = Some(writer);
        self.current_file_path = Some(file_path.clone());
        self.current_file_packet_count = 0;
        self.file_sequence += 1;
        self.created_files.push(file_path); // 移动所有权，减少克隆

        // 标记总大小缓存为无效
        *self.total_size_cache.borrow_mut() = None;

        info!(
            "已创建新文件: {:?}",
            self.current_file_path.as_ref().unwrap()
        );
        Ok(())
    }

    /// 确保当前文件写入器已准备好
    fn ensure_writer_ready(&mut self) -> Result<()> {
        if self.current_writer.is_none() {
            self.create_new_file()?;
        }

        // 检查是否需要切换到新文件
        if self.current_file_packet_count >= self.configuration.max_packets_per_file {
            self.create_new_file()?;
        }

        Ok(())
    }

    /// 更新数据集统计信息（单个数据包）
    fn update_statistics(&mut self, packet: &DataPacket) {
        let timestamp = packet.get_timestamp_ns();

        // 优化时间戳比较，避免unwrap
        match self.first_timestamp {
            None => self.first_timestamp = Some(timestamp),
            Some(first) if timestamp < first => self.first_timestamp = Some(timestamp),
            _ => {}
        }

        match self.last_timestamp {
            None => self.last_timestamp = Some(timestamp),
            Some(last) if timestamp > last => self.last_timestamp = Some(timestamp),
            _ => {}
        }

        // 更新计数
        self.total_packet_count += 1;
        self.current_file_packet_count += 1;

        // 标记统计信息需要更新
        self.statistics_dirty = true;
    }

    /// 批量更新数据集统计信息
    fn update_statistics_batch(&mut self, packets: &[DataPacket]) {
        if packets.is_empty() {
            return;
        }

        // 批量计算时间戳范围
        let timestamps: Vec<u64> = packets.iter().map(|p| p.get_timestamp_ns()).collect();

        if let Some(&min_ts) = timestamps.iter().min() {
            match self.first_timestamp {
                None => self.first_timestamp = Some(min_ts),
                Some(first) if min_ts < first => self.first_timestamp = Some(min_ts),
                _ => {}
            }
        }

        if let Some(&max_ts) = timestamps.iter().max() {
            match self.last_timestamp {
                None => self.last_timestamp = Some(max_ts),
                Some(last) if max_ts > last => self.last_timestamp = Some(max_ts),
                _ => {}
            }
        }

        // 批量更新计数
        self.total_packet_count += packets.len() as u64;
        self.current_file_packet_count += packets.len();

        // 标记统计信息需要更新
        self.statistics_dirty = true;
    }

    /// 更新数据集信息（延迟更新）
    fn update_dataset_info(&mut self) {
        if self.statistics_dirty {
            self.dataset_info.total_packets = self.total_packet_count;
            self.dataset_info.start_timestamp = self.first_timestamp;
            self.dataset_info.end_timestamp = self.last_timestamp;
            self.dataset_info.file_count = self.created_files.len();
            self.statistics_dirty = false;
        }
    }

    /// 更新总大小缓存
    fn update_total_size_cache(&mut self) {
        if self.total_size_cache.borrow().is_none() {
            let mut total_size = 0;
            for file_path in &self.created_files {
                if let Ok(metadata) = std::fs::metadata(file_path) {
                    total_size += metadata.len();
                }
            }
            *self.total_size_cache.borrow_mut() = Some(total_size);
        }
    }

    /// 预计算批量写入需要的文件数量
    fn calculate_files_needed(&self, packet_count: usize) -> usize {
        if packet_count == 0 {
            return 0;
        }

        let remaining_capacity = self
            .configuration
            .max_packets_per_file
            .saturating_sub(self.current_file_packet_count);

        if packet_count <= remaining_capacity {
            0 // 不需要新文件
        } else {
            let additional_packets = packet_count - remaining_capacity;
            1 + (additional_packets - 1) / self.configuration.max_packets_per_file
        }
    }

    /// 完成写入并生成索引
    pub fn finalize(&mut self) -> Result<()> {
        if self.is_finalized {
            return Ok(());
        }

        // 刷新当前文件
        if let Some(ref mut writer) = self.current_writer {
            writer.flush()?;
            writer.close();
        }

        // 更新缓存和统计信息
        self.update_dataset_info();
        self.update_total_size_cache();

        // 生成PIDX索引（如果启用）
        if self.configuration.enable_index_cache {
            self.generate_index()?;
        }

        self.is_finalized = true;
        info!(
            "数据集写入已完成，总文件数: {}, 总数据包数: {}",
            self.created_files.len(),
            self.total_packet_count
        );

        Ok(())
    }

    /// 生成PIDX索引文件
    fn generate_index(&mut self) -> Result<()> {
        if self.created_files.is_empty() {
            return Ok(());
        }

        info!("开始生成PIDX索引...");

        // 使用PidxWriter生成并保存索引
        use crate::business::index::writer::PidxWriter;

        // 创建PidxWriter实例并生成索引
        let mut pidx_writer = PidxWriter::new(&self.dataset_path())?;
        let index_path = pidx_writer.generate_index()?;

        info!("PIDX索引已生成并保存: {:?}", index_path);

        Ok(())
    }
}

impl Write for Writer {
    fn write_packet(&mut self, packet: &DataPacket) -> Result<()> {
        if self.is_finalized {
            return Err(PcapError::InvalidState(ERR_WRITER_FINALIZED.to_string()));
        }

        // 验证数据包
        if !packet.is_valid() {
            return Err(PcapError::CorruptedData(ERR_CORRUPTED_DATA.to_string()));
        }

        // 确保写入器已准备好
        self.ensure_writer_ready()?;

        // 写入数据包
        {
            let writer = self
                .current_writer
                .as_mut()
                .ok_or_else(|| PcapError::InvalidState(ERR_WRITER_NOT_INIT.to_string()))?;

            writer.write_packet(packet)?;

            // 自动刷新（如果启用）
            if self.configuration.auto_flush {
                writer.flush()?;
            }
        }

        // 更新统计信息（在借用结束后）
        self.update_statistics(packet);

        debug!("已写入数据包，总计: {}", self.total_packet_count);
        Ok(())
    }

    fn write_packets(&mut self, packets: &[DataPacket]) -> Result<()> {
        if packets.is_empty() {
            return Ok(());
        }

        // 验证所有数据包
        for packet in packets {
            if !packet.is_valid() {
                return Err(PcapError::CorruptedData(ERR_CORRUPTED_DATA.to_string()));
            }
        }

        // 预计算需要的文件数量
        let files_needed = self.calculate_files_needed(packets.len());

        // 预先创建所需的文件
        for _ in 0..files_needed {
            self.create_new_file()?;
        }

        // 确保写入器已准备好
        self.ensure_writer_ready()?;

        // 批量写入数据包
        for packet in packets {
            let writer = self
                .current_writer
                .as_mut()
                .ok_or_else(|| PcapError::InvalidState(ERR_WRITER_NOT_INIT.to_string()))?;

            writer.write_packet(packet)?;

            // 检查是否需要切换文件
            if self.current_file_packet_count >= self.configuration.max_packets_per_file {
                self.create_new_file()?;
            }
        }

        // 批量更新统计信息
        self.update_statistics_batch(packets);

        // 自动刷新（如果启用）
        if self.configuration.auto_flush {
            if let Some(ref mut writer) = self.current_writer {
                writer.flush()?;
            }
        }

        debug!(
            "已批量写入 {} 个数据包，总计: {}",
            packets.len(),
            self.total_packet_count
        );
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        if let Some(ref mut writer) = self.current_writer {
            writer.flush()?;
        }

        debug!("缓冲区已刷新");
        Ok(())
    }
}

impl Info for Writer {
    fn dataset_info(&self) -> DatasetInfo {
        let mut info = self.dataset_info.clone();

        // 更新统计信息
        info.total_packets = self.total_packet_count;
        info.file_count = self.created_files.len();
        info.start_timestamp = self.first_timestamp;
        info.end_timestamp = self.last_timestamp;

        // 计算总大小
        if let Some(cached_size) = *self.total_size_cache.borrow() {
            info.total_size = cached_size;
        } else {
            let mut total_size = 0;
            for file_path in &self.created_files {
                if let Ok(metadata) = std::fs::metadata(file_path) {
                    total_size += metadata.len();
                }
            }
            info.total_size = total_size;
            *self.total_size_cache.borrow_mut() = Some(total_size);
        }

        info
    }

    fn detailed_file_list(&self) -> Vec<FileInfo> {
        let mut file_infos = Vec::with_capacity(self.created_files.len());

        for file_path in &self.created_files {
            if let Ok(file_info) = FileInfo::from_file(file_path) {
                file_infos.push(file_info);
            }
        }

        file_infos
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        if !self.is_finalized {
            if let Err(e) = self.finalize() {
                warn!("写入器析构时完成操作失败: {}", e);
            }
        }
    }
}

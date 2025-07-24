//! 数据集读取器模块
//!
//! 提供高级的数据集读取功能，支持多文件PCAP数据集的统一读取接口。

use log::{debug, info, warn};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::business::config::Configuration;
use crate::foundation::error::{PcapError, Result};
use crate::data::file_reader::PcapFileReader;
use crate::business::index::{PidxIndex, PidxReader};
use crate::data::models::{DataPacket, DatasetInfo, FileInfo};
use crate::foundation::traits::{Info, Read};
use crate::business::cache::{CacheStats, FileInfoCache};

// 错误消息常量
const ERR_READER_FINALIZED: &str = "读取器已完成，无法继续读取";

/// 数据集读取器
///
/// 提供对整个PCAP数据集的统一读取接口，支持多文件自动切换、索引查询等功能。
pub struct PcapReader {
    /// 基础路径
    base_path: PathBuf,
    /// 数据集名称
    dataset_name: String,
    /// 配置信息（使用 Arc 避免克隆）
    configuration: Arc<Configuration>,
    /// PCAP文件列表（按时间顺序排列）
    pcap_files: Vec<PathBuf>,
    /// 当前文件索引
    current_file_index: usize,
    /// 当前文件读取器
    current_reader: Option<PcapFileReader>,
    /// PIDX索引
    pidx_index: Option<PidxIndex>,
    /// 文件信息缓存
    file_info_cache: FileInfoCache,
    /// 数据集信息
    dataset_info: DatasetInfo,
    /// 当前读取位置（数据包索引）
    current_position: u64,
    /// 第一个数据包时间戳
    first_timestamp: Option<u64>,
    /// 最后一个数据包时间戳
    last_timestamp: Option<u64>,
    /// 总大小缓存（使用RefCell实现内部可变性）
    total_size_cache: RefCell<Option<u64>>,
    /// 缓存统计
    cache_stats: CacheStats,
    /// 是否已完成初始化
    is_initialized: bool,
    /// 是否已完成
    is_finalized: bool,
}

impl PcapReader {
    /// 创建新的数据集读取器
    ///
    /// # 参数
    /// - `dataset_path` - 数据集目录路径
    /// - `config` - PCAP配置
    ///
    /// # 返回
    /// 返回初始化后的读取器实例
    pub fn new<P: AsRef<Path>>(dataset_path: P, config: Configuration) -> Result<Self> {
        let path = dataset_path.as_ref().to_path_buf();

        // 验证数据集目录存在
        if !path.exists() {
            return Err(PcapError::DirectoryNotFound(format!(
                "数据集目录不存在: {:?}",
                path
            )));
        }

        if !path.is_dir() {
            return Err(PcapError::InvalidArgument(format!(
                "指定路径不是目录: {:?}",
                path
            )));
        }

        // 获取数据集名称
        let dataset_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let base_path = path.parent().unwrap_or(&path).to_path_buf();

        // 初始化数据集信息
        let mut dataset_info = DatasetInfo::new(dataset_name.clone(), &path);

        // 扫描PCAP文件
        let pcap_files = Self::scan_pcap_files(&path)?;
        if pcap_files.is_empty() {
            warn!("数据集目录中未找到PCAP文件: {:?}", path);
        }

        // 初始化文件信息缓存
        let file_info_cache = if config.enable_index_cache {
            FileInfoCache::new(config.index_cache_size)
        } else {
            FileInfoCache::new(0)
        };

        // 尝试加载PIDX索引
        let pidx_index = Self::load_pidx_index(&path)?;

        // 更新数据集信息
        dataset_info.file_count = pcap_files.len();
        if let Some(ref index) = pidx_index {
            dataset_info.total_packets = index.total_packets;
            dataset_info.start_timestamp = Some(index.start_timestamp);
            dataset_info.end_timestamp = Some(index.end_timestamp);
            dataset_info.has_index = true;
        }

        let total_packets = dataset_info.total_packets;

        // 创建PcapReader实例
        let mut reader = Self {
            base_path,
            dataset_name,
            configuration: Arc::new(config),
            pcap_files,
            current_file_index: 0,
            current_reader: None,
            pidx_index,
            file_info_cache,
            dataset_info,
            current_position: 0,
            first_timestamp: None,
            last_timestamp: None,
            total_size_cache: RefCell::new(None),
            cache_stats: CacheStats::new(),
            is_initialized: false,
            is_finalized: false,
        };

        // 自动初始化
        reader.auto_initialize()?;

        info!(
            "PcapReader已创建: {:?}, 文件数: {}, 数据包总数: {}",
            path,
            reader.pcap_files.len(),
            total_packets
        );

        Ok(reader)
    }

    /// 获取数据集完整路径
    fn dataset_path(&self) -> PathBuf {
        self.base_path.join(&self.dataset_name)
    }

    /// 扫描数据集目录中的PCAP文件
    fn scan_pcap_files(dataset_path: &Path) -> Result<Vec<PathBuf>> {
        let mut pcap_files = Vec::new();

        let entries = std::fs::read_dir(dataset_path).map_err(|e| PcapError::Io(e))?;

        for entry in entries {
            let entry = entry.map_err(|e| PcapError::Io(e))?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.to_str() == Some("pcap") {
                        pcap_files.push(path);
                    }
                }
            }
        }

        // 按文件名排序（通常对应时间顺序）
        pcap_files.sort();

        debug!("扫描到 {} 个PCAP文件", pcap_files.len());
        Ok(pcap_files)
    }

    /// 加载PIDX索引
    fn load_pidx_index(dataset_path: &Path) -> Result<Option<PidxIndex>> {
        // 尝试查找现有的PIDX文件
        if let Some(pidx_path) = PidxReader::find_pidx_file(dataset_path)? {
            match PidxReader::load_index(&pidx_path) {
                Ok(index) => {
                    info!("已加载PIDX索引文件: {:?}", pidx_path);
                    return Ok(Some(index));
                }
                Err(e) => {
                    warn!("加载PIDX索引失败: {}", e);
                }
            }
        }

        Ok(None)
    }

    /// 打开指定索引的文件
    fn open_file(&mut self, file_index: usize) -> Result<()> {
        if file_index >= self.pcap_files.len() {
            return Err(PcapError::InvalidArgument(format!(
                "文件索引超出范围: {}",
                file_index
            )));
        }

        // 关闭当前文件
        self.current_reader = None;

        // 打开新文件
        let file_path = &self.pcap_files[file_index];
        let mut reader = PcapFileReader::new((*self.configuration).clone());
        reader.open(file_path)?;

        self.current_reader = Some(reader);
        self.current_file_index = file_index;

        debug!("已打开文件: {:?}", file_path);
        Ok(())
    }

    /// 切换到下一个文件
    fn switch_to_next_file(&mut self) -> Result<bool> {
        if self.current_file_index + 1 >= self.pcap_files.len() {
            // 没有更多文件
            return Ok(false);
        }

        self.open_file(self.current_file_index + 1)?;
        Ok(true)
    }

    /// 确保当前文件已打开
    fn ensure_current_file_open(&mut self) -> Result<()> {
        if self.current_reader.is_none() && !self.pcap_files.is_empty() {
            self.open_file(0)?;
        }
        Ok(())
    }

    /// 更新总大小缓存
    fn update_total_size_cache(&self) {
        if self.total_size_cache.borrow().is_none() {
            let mut total_size = 0;
            for file_path in &self.pcap_files {
                if let Ok(metadata) = std::fs::metadata(file_path) {
                    total_size += metadata.len();
                }
            }
            *self.total_size_cache.borrow_mut() = Some(total_size);
        }
    }

    /// 自动初始化
    fn auto_initialize(&mut self) -> Result<()> {
        if self.is_initialized {
            return Ok(());
        }

        // 预加载部分文件信息
        if self.configuration.enable_index_cache && !self.pcap_files.is_empty() {
            let preload_count = std::cmp::min(5, self.pcap_files.len());
            for i in 0..preload_count {
                let file_path = &self.pcap_files[i];
                if let Ok(file_info) = FileInfo::from_file(file_path) {
                    self.file_info_cache.insert(file_path.clone(), file_info);
                }
            }
        }

        // 更新总大小缓存
        self.update_total_size_cache();

        self.is_initialized = true;
        debug!("数据集读取器已自动初始化完成");
        Ok(())
    }

    /// 内部清理缓存
    fn clear_cache(&mut self) -> Result<()> {
        let _ = self.file_info_cache.clear();
        self.cache_stats = CacheStats::new();
        *self.total_size_cache.borrow_mut() = None;
        debug!("缓存已清理");
        Ok(())
    }

    /// 生成索引（如果需要）
    pub fn generate_index(&mut self) -> Result<PathBuf> {
        if self.pidx_index.is_some() {
            // 索引已存在，返回现有索引文件路径
            let pidx_filename = format!("{}.pidx", self.dataset_name);
            return Ok(self.dataset_path().join(pidx_filename));
        }

        info!("开始生成PIDX索引...");

        // 使用PidxWriter生成索引
        use crate::business::index::writer::PidxWriter;

        let mut pidx_writer = PidxWriter::new(&self.dataset_path())?;
        let index_path = pidx_writer.generate_index()?;

        // 重新加载索引
        self.pidx_index = Self::load_pidx_index(&self.dataset_path())?;

        info!("PIDX索引已生成: {:?}", index_path);
        Ok(index_path)
    }
}

impl Read for PcapReader {
    fn read_packet(&mut self) -> Result<Option<DataPacket>> {
        if self.is_finalized {
            return Err(PcapError::InvalidState(ERR_READER_FINALIZED.to_string()));
        }

        self.ensure_current_file_open()?;

        loop {
            if let Some(ref mut reader) = self.current_reader {
                match reader.read_packet() {
                    Ok(Some(packet)) => {
                        self.current_position += 1;

                        // 更新时间戳范围
                        let timestamp = packet.get_timestamp_ns();
                        match self.first_timestamp {
                            None => self.first_timestamp = Some(timestamp),
                            Some(first) if timestamp < first => {
                                self.first_timestamp = Some(timestamp)
                            }
                            _ => {}
                        }
                        match self.last_timestamp {
                            None => self.last_timestamp = Some(timestamp),
                            Some(last) if timestamp > last => self.last_timestamp = Some(timestamp),
                            _ => {}
                        }

                        return Ok(Some(packet));
                    }
                    Ok(None) => {
                        // 当前文件读取完毕，尝试切换到下一个文件
                        if !self.switch_to_next_file()? {
                            // 没有更多文件
                            return Ok(None);
                        }
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            } else {
                // 没有可读取的文件
                return Ok(None);
            }
        }
    }

    fn read_packets(&mut self, count: usize) -> Result<Vec<DataPacket>> {
        let mut packets = Vec::with_capacity(count);

        for _ in 0..count {
            match self.read_packet()? {
                Some(packet) => packets.push(packet),
                None => break,
            }
        }

        Ok(packets)
    }

    fn reset(&mut self) -> Result<()> {
        self.current_file_index = 0;
        self.current_reader = None;
        self.current_position = 0;

        if !self.pcap_files.is_empty() {
            self.open_file(0)?;
        }

        info!("读取器已重置到数据集开始位置");
        Ok(())
    }
}

impl Info for PcapReader {
    fn dataset_info(&self) -> DatasetInfo {
        let mut info = self.dataset_info.clone();

        // 更新实时信息
        info.start_timestamp = self.first_timestamp;
        info.end_timestamp = self.last_timestamp;

        // 计算总大小
        if let Some(cached_size) = *self.total_size_cache.borrow() {
            info.total_size = cached_size;
        } else {
            self.update_total_size_cache();
            info.total_size = self.total_size_cache.borrow().unwrap_or(0);
        }

        info
    }

    fn detailed_file_list(&self) -> Vec<FileInfo> {
        let mut file_infos = Vec::with_capacity(self.pcap_files.len());

        for file_path in &self.pcap_files {
            if let Ok(file_info) = FileInfo::from_file(file_path) {
                file_infos.push(file_info);
            }
        }

        file_infos
    }
}

impl Drop for PcapReader {
    fn drop(&mut self) {
        if !self.is_finalized {
            // 关闭当前文件
            self.current_reader = None;

            // 清理缓存
            let _ = self.clear_cache();

            self.is_finalized = true;
            debug!("数据集读取器已自动完成");
        }
    }
}

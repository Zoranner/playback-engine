//! 数据集读取器模块
//!
//! 提供高级的数据集读取功能，支持多文件PCAP数据集的统一读取接口。

use log::{debug, info};
use std::cell::RefCell;
use std::path::{Path, PathBuf};

use crate::business::cache::{CacheStats, FileInfoCache};
use crate::business::config::ReaderConfig;
use crate::business::index::IndexManager;
use crate::data::models::{
    DataPacket, DatasetInfo, FileInfo,
};
use crate::foundation::error::{PcapError, Result};
use crate::foundation::traits::Read;

// 错误消息常量
const ERROR_DATASET_NOT_FOUND: &str = "数据集目录不存在";
const ERROR_INVALID_DATASET: &str = "无效的数据集目录";

/// PCAP数据集读取器
///
/// 提供对PCAP数据集的高性能读取功能，支持：
/// - 自动索引管理和验证
/// - 顺序读取和文件切换
/// - 智能缓存和性能优化
/// - 多文件数据集统一访问
pub struct PcapReader {
    /// 数据集目录路径
    dataset_path: PathBuf,
    /// 数据集名称
    dataset_name: String,
    /// 索引管理器
    index_manager: IndexManager,
    /// 配置信息
    configuration: ReaderConfig,
    /// 当前文件读取器
    current_reader:
        Option<crate::data::file_reader::PcapFileReader>,
    /// 当前文件索引
    current_file_index: usize,
    /// 当前读取位置（全局数据包索引）
    current_position: u64,
    /// 文件信息缓存
    file_info_cache: FileInfoCache,
    /// 缓存统计信息
    cache_stats: CacheStats,
    /// 总大小缓存
    total_size_cache: RefCell<Option<u64>>,
    /// 是否已初始化
    is_initialized: bool,
}

impl PcapReader {
    /// 创建新的PCAP读取器
    ///
    /// # 参数
    /// - `base_path` - 基础路径
    /// - `dataset_name` - 数据集名称
    ///
    /// # 返回
    /// 返回初始化后的读取器实例
    pub fn new<P: AsRef<Path>>(
        base_path: P,
        dataset_name: &str,
    ) -> Result<Self> {
        Self::new_with_config(
            base_path,
            dataset_name,
            ReaderConfig::default(),
        )
    }

    /// 创建新的PCAP读取器（带配置）
    ///
    /// # 参数
    /// - `base_path` - 基础路径
    /// - `dataset_name` - 数据集名称
    /// - `configuration` - 读取器配置信息
    ///
    /// # 返回
    /// 返回初始化后的读取器实例
    pub fn new_with_config<P: AsRef<Path>>(
        base_path: P,
        dataset_name: &str,
        configuration: ReaderConfig,
    ) -> Result<Self> {
        let dataset_path =
            base_path.as_ref().join(dataset_name);

        // 验证数据集目录
        if !dataset_path.exists() {
            return Err(PcapError::DirectoryNotFound(
                ERROR_DATASET_NOT_FOUND.to_string(),
            ));
        }

        if !dataset_path.is_dir() {
            return Err(PcapError::InvalidArgument(
                ERROR_INVALID_DATASET.to_string(),
            ));
        }

        // 创建索引管理器
        let index_manager =
            IndexManager::new(&dataset_path)?;

        info!(
            "PcapReader已创建 - 数据集: {}",
            dataset_name
        );

        Ok(Self {
            dataset_path,
            dataset_name: dataset_name.to_string(),
            index_manager,
            configuration,
            current_reader: None,
            current_file_index: 0,
            current_position: 0,
            file_info_cache: FileInfoCache::new(1000),
            cache_stats: CacheStats::new(),
            total_size_cache: RefCell::new(None),
            is_initialized: false,
        })
    }

    /// 初始化读取器
    ///
    /// 确保索引可用并准备好读取操作
    pub fn initialize(&mut self) -> Result<()> {
        if self.is_initialized {
            return Ok(());
        }

        info!("初始化PcapReader...");

        // 确保索引可用
        let _index = self.index_manager.ensure_index()?;

        self.is_initialized = true;
        info!("PcapReader初始化完成");
        Ok(())
    }

    /// 获取数据集信息
    pub fn get_dataset_info(
        &mut self,
    ) -> Result<DatasetInfo> {
        self.initialize()?;

        let index = self
            .index_manager
            .get_index()
            .ok_or_else(|| {
                PcapError::InvalidState(
                    "索引未加载".to_string(),
                )
            })?;

        use chrono::Utc;

        Ok(DatasetInfo {
            name: self.dataset_name.clone(),
            path: self.dataset_path.clone(),
            file_count: index.data_files.files.len(),
            total_packets: index.total_packets,
            total_size: self.get_total_size()?,
            start_timestamp: if index.start_timestamp > 0 {
                Some(index.start_timestamp)
            } else {
                None
            },
            end_timestamp: if index.end_timestamp > 0 {
                Some(index.end_timestamp)
            } else {
                None
            },
            created_time: Utc::now().to_rfc3339(),
            modified_time: Utc::now().to_rfc3339(),
            has_index: true,
        })
    }

    /// 获取文件信息列表
    pub fn get_file_info_list(
        &mut self,
    ) -> Result<Vec<FileInfo>> {
        self.initialize()?;

        let index = self
            .index_manager
            .get_index()
            .ok_or_else(|| {
                PcapError::InvalidState(
                    "索引未加载".to_string(),
                )
            })?;

        use chrono::Utc;
        let current_time = Utc::now().to_rfc3339();

        let mut file_infos = Vec::new();
        for file_index in &index.data_files.files {
            let file_info = FileInfo {
                file_name: file_index.file_name.clone(),
                file_path: self
                    .dataset_path
                    .join(&file_index.file_name),
                file_size: file_index.file_size,
                packet_count: file_index.packet_count,
                start_timestamp: if file_index
                    .start_timestamp
                    > 0
                {
                    Some(file_index.start_timestamp)
                } else {
                    None
                },
                end_timestamp: if file_index.end_timestamp
                    > 0
                {
                    Some(file_index.end_timestamp)
                } else {
                    None
                },
                file_hash: Some(
                    file_index.file_hash.clone(),
                ),
                created_time: current_time.clone(),
                modified_time: current_time.clone(),
                is_valid: true,
            };
            file_infos.push(file_info);
        }

        Ok(file_infos)
    }

    /// 获取数据集路径
    pub fn dataset_path(&self) -> &Path {
        &self.dataset_path
    }

    /// 获取数据集名称
    pub fn dataset_name(&self) -> &str {
        &self.dataset_name
    }

    /// 获取索引管理器的引用
    /// 允许外部通过 reader.index().method() 的方式访问索引功能
    pub fn index(&self) -> &IndexManager {
        &self.index_manager
    }

    /// 获取索引管理器的可变引用
    /// 允许外部通过 reader.index_mut().method() 的方式访问索引功能
    pub fn index_mut(&mut self) -> &mut IndexManager {
        &mut self.index_manager
    }

    /// 强制重新生成索引
    pub fn regenerate_index(&mut self) -> Result<PathBuf> {
        info!("强制重新生成索引...");
        let index_path =
            self.index_manager.regenerate_index()?;
        info!("索引已重新生成: {:?}", index_path);
        Ok(index_path)
    }

    /// 获取缓存统计信息
    pub fn get_cache_stats(&self) -> &CacheStats {
        &self.cache_stats
    }

    /// 清理缓存
    pub fn clear_cache(&mut self) -> Result<()> {
        let _ = self.file_info_cache.clear();
        self.cache_stats = CacheStats::new();
        *self.total_size_cache.borrow_mut() = None;
        debug!("缓存已清理");
        Ok(())
    }

    // =================================================================
    // 私有方法
    // =================================================================

    /// 获取数据集总大小
    fn get_total_size(&self) -> Result<u64> {
        if let Some(cached_size) =
            *self.total_size_cache.borrow()
        {
            return Ok(cached_size);
        }

        let index = self
            .index_manager
            .get_index()
            .ok_or_else(|| {
                PcapError::InvalidState(
                    "索引未加载".to_string(),
                )
            })?;

        let total_size: u64 = index
            .data_files
            .files
            .iter()
            .map(|f| f.file_size)
            .sum();

        *self.total_size_cache.borrow_mut() =
            Some(total_size);
        Ok(total_size)
    }

    /// 打开指定索引的文件
    fn open_file(
        &mut self,
        file_index: usize,
    ) -> Result<()> {
        let index = self
            .index_manager
            .get_index()
            .ok_or_else(|| {
                PcapError::InvalidState(
                    "索引未加载".to_string(),
                )
            })?;

        if file_index >= index.data_files.files.len() {
            return Err(PcapError::InvalidArgument(
                format!("文件索引超出范围: {}", file_index),
            ));
        }

        // 关闭当前文件
        if let Some(ref mut reader) = self.current_reader {
            reader.close();
        }

        // 打开新文件
        let file_info = &index.data_files.files[file_index];
        let file_path =
            self.dataset_path.join(&file_info.file_name);

        let mut reader =
            crate::data::file_reader::PcapFileReader::new(
                self.configuration.common.clone(),
            );
        reader.open(&file_path)?;

        self.current_reader = Some(reader);
        self.current_file_index = file_index;

        debug!("已打开文件: {:?}", file_path);
        Ok(())
    }

    /// 切换到下一个文件
    fn switch_to_next_file(&mut self) -> Result<bool> {
        let index = self
            .index_manager
            .get_index()
            .ok_or_else(|| {
                PcapError::InvalidState(
                    "索引未加载".to_string(),
                )
            })?;

        if self.current_file_index + 1
            >= index.data_files.files.len()
        {
            // 没有更多文件
            return Ok(false);
        }

        self.open_file(self.current_file_index + 1)?;
        Ok(true)
    }

    /// 确保当前文件已打开
    fn ensure_current_file_open(&mut self) -> Result<()> {
        if self.current_reader.is_none() {
            let index = self
                .index_manager
                .get_index()
                .ok_or_else(|| {
                    PcapError::InvalidState(
                        "索引未加载".to_string(),
                    )
                })?;

            if !index.data_files.files.is_empty() {
                self.open_file(0)?;
            }
        }
        Ok(())
    }
}

impl Read for PcapReader {
    fn read_packet(
        &mut self,
    ) -> Result<Option<DataPacket>> {
        self.initialize()?;

        // 确保当前文件已打开
        self.ensure_current_file_open()?;

        loop {
            if let Some(ref mut reader) =
                self.current_reader
            {
                match reader.read_packet() {
                    Ok(Some(packet)) => {
                        self.current_position += 1;
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

    fn read_packets(
        &mut self,
        count: usize,
    ) -> Result<Vec<DataPacket>> {
        self.initialize()?;

        let mut packets = Vec::with_capacity(count);

        // 批量读取指定数量的数据包
        for _ in 0..count {
            if let Some(packet) = self.read_packet()? {
                packets.push(packet);
            } else {
                break; // 没有更多数据包
            }
        }

        Ok(packets)
    }

    fn reset(&mut self) -> Result<()> {
        self.initialize()?;

        // 重置当前读取位置到数据集开始
        self.current_position = 0;
        self.current_file_index = 0;

        // 关闭当前文件
        if let Some(ref mut reader) = self.current_reader {
            reader.close();
        }
        self.current_reader = None;

        // 重新打开第一个文件（如果存在）
        let index = self
            .index_manager
            .get_index()
            .ok_or_else(|| {
                PcapError::InvalidState(
                    "索引未加载".to_string(),
                )
            })?;

        if !index.data_files.files.is_empty() {
            self.open_file(0)?;
        }

        info!("读取器已重置到数据集开始位置");
        Ok(())
    }
}

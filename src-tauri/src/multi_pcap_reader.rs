use std::collections::HashMap;
use std::path::{Path, PathBuf};
use log::{debug, info, warn};

use crate::types::{DataPacket, PlaybackError, Result};
use crate::pcap_reader::PcapReader;
use crate::pidx::{PidxManager, PidxIndex, PacketIndexEntry};

/// 多文件PCAP读取器
/// 支持基于时间索引的跨文件数据包查找和读取
pub struct MultiPcapReader {
    /// 数据集根目录
    dataset_path: PathBuf,
    /// PIDX时间索引
    index: PidxIndex,
    /// 当前打开的PCAP文件读取器缓存
    reader_cache: HashMap<String, PcapReader>,
    /// 缓存大小限制
    max_cache_size: usize,
    /// 当前读取位置（时间戳）
    current_timestamp: u64,
}

impl MultiPcapReader {
    /// 创建新的多文件PCAP读取器
    ///
    /// # 参数
    /// * `dataset_path` - 数据集目录路径
    /// * `index` - PIDX时间索引
    pub fn new<P: AsRef<Path>>(dataset_path: P, index: PidxIndex) -> Result<Self> {
        let path = dataset_path.as_ref().to_path_buf();

        info!("创建多文件PCAP读取器，数据集: {:?}", path);
        info!("索引包含 {} 个文件，{} 个数据包",
              index.files.len(), index.total_packets);

        Ok(Self {
            dataset_path: path,
            index,
            reader_cache: HashMap::new(),
            max_cache_size: 5, // 最多缓存5个文件读取器
            current_timestamp: 0,
        })
    }

    /// 从数据集目录和PIDX文件创建读取器
    pub async fn from_dataset<P: AsRef<Path>>(dataset_path: P) -> Result<Self> {
        let path = dataset_path.as_ref();

        // 查找PIDX文件
        let pidx_file_path = Self::find_pidx_file(path)?;

        let index = if let Some(pidx_path) = pidx_file_path {
            info!("找到PIDX文件: {:?}", pidx_path);

            // 加载现有索引
            let index = PidxManager::load_index(&pidx_path)?;

            // 验证索引有效性
            if PidxManager::verify_index_validity(&index, path).await? {
                info!("PIDX索引验证通过，使用现有索引");
                index
            } else {
                warn!("PIDX索引验证失败，重新生成索引");
                let new_index = PidxManager::generate_index(path).await?;

                // 保存新索引
                let new_pidx_path = Self::get_pidx_file_path(path);
                PidxManager::save_index(&new_index, &new_pidx_path)?;

                new_index
            }
        } else {
            info!("未找到PIDX文件，生成新索引");
            let new_index = PidxManager::generate_index(path).await?;

            // 保存新索引
            let pidx_path = Self::get_pidx_file_path(path);
            PidxManager::save_index(&new_index, &pidx_path)?;

            new_index
        };

        Self::new(path, index)
    }

    /// 查找数据集目录中的PIDX文件
    fn find_pidx_file<P: AsRef<Path>>(dataset_path: P) -> Result<Option<PathBuf>> {
        let path = dataset_path.as_ref();
        let dataset_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("dataset");

        let pidx_file_name = format!("{}.pidx", dataset_name);
        let pidx_path = path.join(pidx_file_name);

        if pidx_path.exists() {
            Ok(Some(pidx_path))
        } else {
            Ok(None)
        }
    }

    /// 获取PIDX文件路径
    pub fn get_pidx_file_path<P: AsRef<Path>>(dataset_path: P) -> PathBuf {
        let path = dataset_path.as_ref();
        let dataset_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("dataset");

        let pidx_file_name = format!("{}.pidx", dataset_name);
        path.join(pidx_file_name)
    }

        /// 根据时间戳查找并读取数据包
    pub fn read_packet_at_time(&mut self, timestamp: u64) -> Result<Option<DataPacket>> {
        // 在索引中查找数据包位置
        if let Some(entry) = self.index.find_packet_by_timestamp(timestamp) {
            let entry_clone = entry.clone();
            self.read_packet_from_entry(&entry_clone)
        } else {
            debug!("未找到时间戳为 {} 的数据包", timestamp);
            Ok(None)
        }
    }

    /// 根据索引条目读取数据包
    fn read_packet_from_entry(&mut self, entry: &PacketIndexEntry) -> Result<Option<DataPacket>> {
        // 获取或创建文件读取器
        let reader = self.get_or_create_reader(&entry.file_name)?;

        // 跳转到指定位置
        reader.seek_to_byte_position(entry.byte_offset)?;

        // 读取数据包
        reader.read_next_packet()
    }

        /// 读取指定时间范围内的所有数据包
    pub fn read_packets_in_range(&mut self, start_time: u64, end_time: u64) -> Result<Vec<DataPacket>> {
        let packet_entries: Vec<_> = self.index.get_packets_in_range(start_time, end_time)
            .into_iter()
            .cloned()
            .collect();
        let mut packets = Vec::new();

        debug!("读取时间范围 {} - {} 内的 {} 个数据包",
               start_time, end_time, packet_entries.len());

        for entry in packet_entries {
            if let Some(packet) = self.read_packet_from_entry(&entry)? {
                packets.push(packet);
            }
        }

        // 按时间戳排序
        packets.sort_by_key(|p| p.get_timestamp_ns());

        Ok(packets)
    }

    /// 顺序读取下一个数据包
    pub fn read_next_packet(&mut self) -> Result<Option<DataPacket>> {
        // 查找当前时间戳之后的第一个数据包
        let next_timestamp = self.find_next_timestamp(self.current_timestamp)?;

        if let Some(timestamp) = next_timestamp {
            self.current_timestamp = timestamp;
            self.read_packet_at_time(timestamp)
        } else {
            Ok(None) // 没有更多数据包
        }
    }

    /// 查找指定时间戳之后的下一个时间戳
    fn find_next_timestamp(&self, current_time: u64) -> Result<Option<u64>> {
        let mut min_next_timestamp: Option<u64> = None;

        for timestamp in self.index.timestamp_index.keys() {
            if *timestamp > current_time {
                match min_next_timestamp {
                    None => min_next_timestamp = Some(*timestamp),
                    Some(min_time) => {
                        if *timestamp < min_time {
                            min_next_timestamp = Some(*timestamp);
                        }
                    }
                }
            }
        }

        Ok(min_next_timestamp)
    }

    /// 跳转到指定时间点
    pub fn seek_to_time(&mut self, target_time: u64) -> Result<()> {
        self.current_timestamp = target_time;
        debug!("跳转到时间点: {} ns", target_time);
        Ok(())
    }

    /// 重置到开始位置
    pub fn reset(&mut self) -> Result<()> {
        self.current_timestamp = self.index.start_timestamp;
        debug!("重置到开始时间: {} ns", self.current_timestamp);
        Ok(())
    }

    /// 获取或创建文件读取器
    fn get_or_create_reader(&mut self, file_name: &str) -> Result<&mut PcapReader> {
        // 检查缓存中是否已存在
        if !self.reader_cache.contains_key(file_name) {
            // 缓存大小控制
            if self.reader_cache.len() >= self.max_cache_size {
                // 移除最旧的读取器（简单实现：移除第一个）
                if let Some(oldest_key) = self.reader_cache.keys().next().cloned() {
                    self.reader_cache.remove(&oldest_key);
                    debug!("从缓存中移除读取器: {}", oldest_key);
                }
            }

            // 创建新的读取器
            let file_path = self.dataset_path.join(file_name);
            let reader = PcapReader::new(&file_path)?;
            self.reader_cache.insert(file_name.to_string(), reader);

            debug!("创建新的文件读取器: {}", file_name);
        }

        Ok(self.reader_cache.get_mut(file_name).unwrap())
    }

    /// 获取数据集的总时长（纳秒）
    pub fn get_total_duration(&self) -> u64 {
        self.index.total_duration
    }

    /// 获取数据包总数
    pub fn get_total_packets(&self) -> u64 {
        self.index.total_packets
    }

    /// 获取开始时间戳
    pub fn get_start_timestamp(&self) -> u64 {
        self.index.start_timestamp
    }

    /// 获取结束时间戳
    pub fn get_end_timestamp(&self) -> u64 {
        self.index.end_timestamp
    }

    /// 获取当前时间戳
    pub fn get_current_timestamp(&self) -> u64 {
        self.current_timestamp
    }

    /// 获取数据集路径
    pub fn get_dataset_path(&self) -> &Path {
        &self.dataset_path
    }

    /// 获取文件列表
    pub fn get_file_list(&self) -> Vec<&str> {
        self.index.files.iter().map(|f| f.file_name.as_str()).collect()
    }

    /// 获取指定文件的信息
    pub fn get_file_info(&self, file_name: &str) -> Option<&crate::pidx::PcapFileIndex> {
        self.index.files.iter().find(|f| f.file_name == file_name)
    }

    /// 清空读取器缓存
    pub fn clear_cache(&mut self) {
        self.reader_cache.clear();
        debug!("清空读取器缓存");
    }

    /// 设置缓存大小限制
    pub fn set_cache_size(&mut self, size: usize) {
        self.max_cache_size = size;

        // 如果当前缓存超过新限制，移除多余的读取器
        while self.reader_cache.len() > self.max_cache_size {
            if let Some(key) = self.reader_cache.keys().next().cloned() {
                self.reader_cache.remove(&key);
            }
        }

        debug!("设置缓存大小限制: {}", size);
    }

    /// 获取索引信息
    pub fn get_index(&self) -> &PidxIndex {
        &self.index
    }
}

// 注意：字节位置跳转功能已在PcapReader中实现
// 无需额外的trait扩展

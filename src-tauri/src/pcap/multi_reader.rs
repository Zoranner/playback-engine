use std::collections::HashMap;
use std::path::{Path, PathBuf};
use log::{debug, info, warn};

use crate::types::{DataPacket, PlaybackError, Result};
use crate::types::{PidxIndex, PacketIndexEntry};
use crate::pcap::reader::PcapReader;

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
        let pidx_files: Vec<_> = std::fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(false, |ext| ext == "pidx")
            })
            .collect();

        if pidx_files.is_empty() {
            return Err(PlaybackError::ProjectError(
                "数据集目录中未找到PIDX索引文件".to_string()
            ));
        }

        if pidx_files.len() > 1 {
            warn!("发现多个PIDX文件，使用第一个: {:?}", pidx_files[0].path());
        }

        // 读取PIDX索引
        let pidx_path = pidx_files[0].path();
        let pidx_content = std::fs::read_to_string(&pidx_path)?;
        let index: PidxIndex = serde_json::from_str(&pidx_content)?;

        Self::new(path, index)
    }

    /// 根据时间戳查找对应的数据包索引条目
    fn find_packet_by_timestamp(&self, timestamp: u64) -> Option<&PacketIndexEntry> {
        // 在所有文件的所有数据包中搜索最接近的时间戳
        let mut best_match: Option<&PacketIndexEntry> = None;
        let mut min_diff = u64::MAX;

        for file in &self.index.files {
            // 如果时间戳不在这个文件的范围内，跳过
            if timestamp < file.start_timestamp || timestamp > file.end_timestamp {
                continue;
            }

            // 在文件内搜索最接近的数据包
            for packet in &file.packets {
                let diff = if packet.timestamp_ns <= timestamp {
                    timestamp - packet.timestamp_ns
                } else {
                    packet.timestamp_ns - timestamp
                };

                if diff < min_diff {
                    min_diff = diff;
                    best_match = Some(packet);
                }
            }
        }

        best_match
    }

    /// 获取或创建文件读取器
    fn get_or_create_reader(&mut self, file_name: &str) -> Result<&mut PcapReader> {
        // 如果缓存中没有，创建新的读取器
        if !self.reader_cache.contains_key(file_name) {
            // 检查缓存大小，如果超过限制则清理最老的
            if self.reader_cache.len() >= self.max_cache_size {
                // 简单的策略：清空所有缓存
                // TODO: 可以实现LRU策略
                self.reader_cache.clear();
                debug!("清理PCAP读取器缓存");
            }

            let file_path = self.dataset_path.join(file_name);
            let reader = PcapReader::new(file_path)?;
            self.reader_cache.insert(file_name.to_string(), reader);

            debug!("创建PCAP读取器: {}", file_name);
        }

        Ok(self.reader_cache.get_mut(file_name).unwrap())
    }

    /// 读取指定时间戳的数据包
    pub fn read_packet_at_time(&mut self, timestamp: u64) -> Result<Option<DataPacket>> {
        // 查找对应的数据包索引
        let packet_index = match self.find_packet_by_timestamp(timestamp) {
            Some(index) => index.clone(),
            None => {
                debug!("未找到时间戳 {} 对应的数据包", timestamp);
                return Ok(None);
            }
        };

        debug!("找到数据包: 文件={}, 偏移={}, 时间戳={}",
               packet_index.file_name, packet_index.byte_offset, packet_index.timestamp_ns);

        // 获取对应的文件读取器
        let reader = self.get_or_create_reader(&packet_index.file_name)?;

        // 跳转到指定位置
        reader.seek_to_byte_position(packet_index.byte_offset)?;

        // 读取数据包
        reader.read_next_packet()
    }

    /// 跳转到指定时间戳
    pub fn seek_to_timestamp(&mut self, timestamp: u64) -> Result<()> {
        self.current_timestamp = timestamp;
        debug!("跳转到时间戳: {} ns", timestamp);
        Ok(())
    }

    /// 读取下一个数据包（按时间顺序）
    pub fn read_next_packet(&mut self) -> Result<Option<DataPacket>> {
        // 查找当前时间戳之后的下一个数据包
        let mut next_packet: Option<&PacketIndexEntry> = None;
        let mut min_timestamp = u64::MAX;

        for file in &self.index.files {
            for packet in &file.packets {
                if packet.timestamp_ns > self.current_timestamp && packet.timestamp_ns < min_timestamp {
                    min_timestamp = packet.timestamp_ns;
                    next_packet = Some(packet);
                }
            }
        }

        if let Some(packet_index) = next_packet {
            let packet_index = packet_index.clone();
            self.current_timestamp = packet_index.timestamp_ns;

            // 获取对应的文件读取器
            let reader = self.get_or_create_reader(&packet_index.file_name)?;

            // 跳转到指定位置
            reader.seek_to_byte_position(packet_index.byte_offset)?;

            // 读取数据包
            reader.read_next_packet()
        } else {
            Ok(None) // 没有更多数据包
        }
    }

    /// 获取数据集的总时长
    pub fn get_total_duration(&self) -> u64 {
        if self.index.end_timestamp > self.index.start_timestamp {
            self.index.end_timestamp - self.index.start_timestamp
        } else {
            0
        }
    }

    /// 获取数据包总数
    pub fn get_total_packets(&self) -> u64 {
        self.index.total_packets
    }

    /// 获取当前时间戳
    pub fn get_current_timestamp(&self) -> u64 {
        self.current_timestamp
    }

    /// 获取索引信息
    pub fn get_index(&self) -> &PidxIndex {
        &self.index
    }

    /// 重置到开始位置
    pub fn reset(&mut self) -> Result<()> {
        self.current_timestamp = self.index.start_timestamp;
        debug!("多文件PCAP读取器已重置到开始位置");
        Ok(())
    }

    /// 清理缓存
    pub fn clear_cache(&mut self) {
        self.reader_cache.clear();
        debug!("清理所有PCAP读取器缓存");
    }
}

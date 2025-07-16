use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 单个数据包在索引中的记录
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "packet")]
pub struct PacketIndexEntry {
    /// 时间戳（纳秒）
    pub timestamp_ns: u64,
    /// 所在文件名
    pub file_name: String,
    /// 在文件中的字节位置
    pub byte_offset: u64,
    /// 数据包大小
    pub packet_size: u32,
}

/// 单个PCAP文件的索引信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "file")]
pub struct PcapFileIndex {
    /// 文件名
    pub file_name: String,
    /// 文件SHA256哈希值
    pub file_hash: String,
    /// 文件大小（字节）
    pub file_size: u64,
    /// 数据包数量
    pub packet_count: u64,
    /// 文件中第一个数据包的时间戳
    pub start_timestamp: u64,
    /// 文件中最后一个数据包的时间戳
    pub end_timestamp: u64,
    /// 该文件中所有数据包的索引条目
    #[serde(rename = "packet")]
    pub packets: Vec<PacketIndexEntry>,
}

/// PIDX索引文件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "pidx_index")]
pub struct PidxIndex {
    /// 索引版本
    pub version: String,
    /// 索引描述
    pub description: Option<String>,
    /// 创建时间
    pub created_time: String,
    /// 数据集开始时间戳
    pub start_timestamp: u64,
    /// 数据集结束时间戳
    pub end_timestamp: u64,
    /// 数据包总数
    pub total_packets: u64,
    /// 总时长（纳秒）
    pub total_duration: u64,
    /// 包含的PCAP文件索引
    #[serde(rename = "file")]
    pub files: Vec<PcapFileIndex>,
    /// 时间戳到文件位置的快速查找映射（不序列化）
    #[serde(skip)]
    pub timestamp_index: HashMap<u64, PacketIndexEntry>,
}

impl PidxIndex {
    /// 创建新的索引
    pub fn new(description: Option<String>) -> Self {
        use chrono::Utc;

        Self {
            version: "1.0.0".to_string(),
            description,
            created_time: Utc::now().to_rfc3339(),
            start_timestamp: 0,
            end_timestamp: 0,
            total_packets: 0,
            total_duration: 0,
            files: Vec::new(),
            timestamp_index: HashMap::new(),
        }
    }

    /// 更新时间范围
    pub fn update_time_range(&mut self) {
        if self.files.is_empty() {
            self.start_timestamp = 0;
            self.end_timestamp = 0;
            self.total_duration = 0;
            return;
        }

        self.start_timestamp = self.files.iter()
            .map(|f| f.start_timestamp)
            .min()
            .unwrap_or(0);

        self.end_timestamp = self.files.iter()
            .map(|f| f.end_timestamp)
            .max()
            .unwrap_or(0);

        self.total_duration = if self.end_timestamp > self.start_timestamp {
            self.end_timestamp - self.start_timestamp
        } else {
            0
        };
    }

    /// 更新数据包总数
    pub fn update_total_packets(&mut self) {
        self.total_packets = self.files.iter()
            .map(|f| f.packet_count)
            .sum();
    }

    /// 构建时间戳快速查找索引
    pub fn build_timestamp_index(&mut self) {
        self.timestamp_index.clear();

        for file_index in &self.files {
            for packet in &file_index.packets {
                self.timestamp_index.insert(packet.timestamp_ns, packet.clone());
            }
        }

        log::debug!("构建时间戳索引完成，包含 {} 个条目", self.timestamp_index.len());
    }
}

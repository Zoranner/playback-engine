use serde::{Deserialize, Serialize};

/// 数据包类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PacketType {
    Environment,  // 环境信息
    Event,        // 事件信息
    Target,       // 目标信息
    Unknown,      // 未知类型
}

/// PCAP数据包结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPacket {
    /// 时间戳秒部分
    pub timestamp_sec: u32,
    /// 时间戳纳秒部分  
    pub timestamp_nsec: u32,
    /// 数据内容
    pub data: Vec<u8>,
    /// 数据包类型
    pub packet_type: PacketType,
    /// 数据包大小
    pub size: u32,
}

impl DataPacket {
    /// 创建新的数据包
    pub fn new(timestamp_sec: u32, timestamp_nsec: u32, data: Vec<u8>, packet_type: PacketType) -> Self {
        let size = data.len() as u32;
        Self {
            timestamp_sec,
            timestamp_nsec,
            data,
            packet_type,
            size,
        }
    }
    
    /// 获取完整时间戳（纳秒）
    pub fn get_timestamp_ns(&self) -> u64 {
        (self.timestamp_sec as u64) * 1_000_000_000 + (self.timestamp_nsec as u64)
    }
}

/// 工程元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub version: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub participants: Vec<String>,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            description: None,
            tags: Vec::new(),
            participants: Vec::new(),
        }
    }
}

/// 工程信息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub path: String,
    pub total_duration: u64,        // 总时长（纳秒）
    pub file_count: usize,
    pub start_time: String,         // ISO格式时间字符串
    pub end_time: String,           // ISO格式时间字符串
    pub metadata: ProjectMetadata,
    pub pcap_files: Vec<String>,    // PCAP文件路径列表
}

impl ProjectInfo {
    /// 创建新的工程信息
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            total_duration: 0,
            file_count: 0,
            start_time: String::new(),
            end_time: String::new(),
            metadata: ProjectMetadata::default(),
            pcap_files: Vec::new(),
        }
    }
}

/// 播放状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub is_paused: bool,
    pub current_time: u64,      // 当前播放时间（纳秒）
    pub total_duration: u64,    // 总时长（纳秒）
    pub speed: f32,             // 播放倍速
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            is_playing: false,
            is_paused: false,
            current_time: 0,
            total_duration: 0,
            speed: 1.0,
        }
    }
}

/// 数据更新负载
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataUpdatePayload {
    pub environment: serde_json::Value,
    pub events: serde_json::Value,
    pub targets: serde_json::Value,
    pub timestamp: u64,
}

/// 错误类型定义
#[derive(thiserror::Error, Debug)]
pub enum PlaybackError {
    #[error("文件读取错误: {0}")]
    FileError(#[from] std::io::Error),
    
    #[error("JSON序列化/反序列化错误: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("文件格式错误: {0}")]
    FormatError(String),
    
    #[error("工程错误: {0}")]
    ProjectError(String),
    
    #[error("播放引擎错误: {0}")]
    PlaybackEngineError(String),
    
    #[error("数据解析错误: {0}")]
    ParseError(String),
}

/// 统一的Result类型
pub type Result<T> = std::result::Result<T, PlaybackError>; 
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// PCAP格式常量定义
pub mod constants {
    /// PCAP文件标识，固定值 0xD4C3B2A1
    pub const PCAP_MAGIC_NUMBER: u32 = 0xD4C3B2A1;

    /// PROJ文件标识 ("PROJ")
    pub const PROJ_MAGIC_NUMBER: u32 = 0xA1B2C3D4;

    /// 主版本号，固定值 0x0002
    pub const MAJOR_VERSION: u16 = 2;

    /// 次版本号，固定值 0x0004，表示支持纳秒级时间量
    pub const MINOR_VERSION: u16 = 4;

    /// 每个PCAP文件最大数据包数量
    pub const DEFAULT_MAX_PACKETS_PER_FILE: usize = 500;

    /// 最大缓冲区大小(字节)
    pub const MAX_BUFFER_SIZE: usize = 50 * 1024 * 1024; // 50MB

    /// 默认文件命名格式
    pub const DEFAULT_FILE_NAME_FORMAT: &str = "yyMMdd_HHmmss_fffffff";

    /// 数据包最大大小(字节)
    pub const MAX_PACKET_SIZE: usize = 30 * 1024 * 1024; // 30MB
}

/// PCAP库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PcapConfiguration {
    /// 每个PCAP文件最大数据包数量
    pub max_packets_per_file: usize,
    /// 缓冲区大小（字节）
    pub buffer_size: usize,
    /// 最大数据包大小（字节）
    pub max_packet_size: usize,
    /// 文件命名格式
    pub file_name_format: String,
    /// 是否启用自动刷新
    pub auto_flush: bool,
    /// 是否启用数据验证
    pub enable_validation: bool,
    /// 是否启用压缩
    pub enable_compression: bool,
    /// 索引缓存大小（条目数）
    pub index_cache_size: usize,
    /// 是否启用文件索引缓存
    pub enable_index_cache: bool,
    /// 索引刷新间隔（毫秒）
    pub index_flush_interval: u64,
    /// 读取超时时间（毫秒）
    pub read_timeout: u64,
    /// 写入超时时间（毫秒）
    pub write_timeout: u64,
    /// 临时目录路径
    pub temp_directory: PathBuf,
}

impl Default for PcapConfiguration {
    fn default() -> Self {
        Self {
            max_packets_per_file: constants::DEFAULT_MAX_PACKETS_PER_FILE,
            buffer_size: 8192,
            max_packet_size: constants::MAX_PACKET_SIZE,
            file_name_format: constants::DEFAULT_FILE_NAME_FORMAT.to_string(),
            auto_flush: true,
            enable_validation: true,
            enable_compression: false,
            index_cache_size: 1000,
            enable_index_cache: true,
            index_flush_interval: 5000,
            read_timeout: 30000,
            write_timeout: 30000,
            temp_directory: std::env::temp_dir(),
        }
    }
}

impl PcapConfiguration {
    /// 获取高性能配置（适用于大量数据处理）
    pub fn high_performance() -> Self {
        Self {
            max_packets_per_file: 2000,
            buffer_size: 64 * 1024, // 64KB
            auto_flush: false,
            index_cache_size: 5000,
            enable_index_cache: true,
            ..Default::default()
        }
    }

    /// 获取低内存配置（适用于内存受限环境）
    pub fn low_memory() -> Self {
        Self {
            max_packets_per_file: 100,
            buffer_size: 2048, // 2KB
            auto_flush: true,
            index_cache_size: 100,
            enable_index_cache: false,
            ..Default::default()
        }
    }

    /// 获取调试配置（启用所有验证和详细日志）
    pub fn debug() -> Self {
        Self {
            max_packets_per_file: 50,
            buffer_size: 4096,
            auto_flush: true,
            enable_validation: true,
            index_cache_size: 50,
            enable_index_cache: true,
            ..Default::default()
        }
    }

    /// 验证配置的有效性
    pub fn validate(&self) -> Result<(), String> {
        if self.max_packets_per_file == 0 {
            return Err("每个文件最大数据包数量必须大于0".to_string());
        }

        if self.buffer_size < 1024 {
            return Err("缓冲区大小不能小于1024字节".to_string());
        }

        if self.buffer_size > constants::MAX_BUFFER_SIZE {
            return Err(format!(
                "缓冲区大小不能超过{}字节",
                constants::MAX_BUFFER_SIZE
            ));
        }

        if self.max_packet_size == 0 {
            return Err("最大数据包大小必须大于0".to_string());
        }

        if self.max_packet_size > constants::MAX_PACKET_SIZE {
            return Err(format!(
                "最大数据包大小不能超过{}字节",
                constants::MAX_PACKET_SIZE
            ));
        }

        if self.index_cache_size == 0 {
            return Err("索引缓存大小必须大于0".to_string());
        }

        if self.file_name_format.is_empty() {
            return Err("文件命名格式不能为空".to_string());
        }

        if !self.temp_directory.exists() {
            return Err("临时目录不存在".to_string());
        }

        Ok(())
    }

    /// 重置为默认值
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// 错误代码枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcapErrorCode {
    /// 未知错误
    Unknown = 0,
    /// 文件未找到
    FileNotFound = 1001,
    /// 目录不存在
    DirectoryNotFound = 1002,
    /// 权限不足
    InsufficientPermissions = 1003,
    /// 磁盘空间不足
    DiskSpaceFull = 1004,
    /// 无效的文件格式
    InvalidFormat = 2001,
    /// 文件头损坏
    CorruptedHeader = 2002,
    /// 数据包损坏
    CorruptedData = 2003,
    /// 校验和不匹配
    ChecksumMismatch = 2004,
    /// 数据包大小无效
    InvalidPacketSize = 3001,
    /// 参数无效
    InvalidArgument = 3002,
    /// 操作状态无效
    InvalidState = 3003,
    /// 缓冲区溢出
    BufferOverflow = 4001,
    /// 内存不足
    OutOfMemory = 4002,
}

impl std::fmt::Display for PcapErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PcapErrorCode::Unknown => write!(f, "未知错误"),
            PcapErrorCode::FileNotFound => write!(f, "文件未找到"),
            PcapErrorCode::DirectoryNotFound => write!(f, "目录不存在"),
            PcapErrorCode::InsufficientPermissions => write!(f, "权限不足"),
            PcapErrorCode::DiskSpaceFull => write!(f, "磁盘空间不足"),
            PcapErrorCode::InvalidFormat => write!(f, "无效的文件格式"),
            PcapErrorCode::CorruptedHeader => write!(f, "文件头损坏"),
            PcapErrorCode::CorruptedData => write!(f, "数据包损坏"),
            PcapErrorCode::ChecksumMismatch => write!(f, "校验和不匹配"),
            PcapErrorCode::InvalidPacketSize => write!(f, "数据包大小无效"),
            PcapErrorCode::InvalidArgument => write!(f, "参数无效"),
            PcapErrorCode::InvalidState => write!(f, "操作状态无效"),
            PcapErrorCode::BufferOverflow => write!(f, "缓冲区溢出"),
            PcapErrorCode::OutOfMemory => write!(f, "内存不足"),
        }
    }
}

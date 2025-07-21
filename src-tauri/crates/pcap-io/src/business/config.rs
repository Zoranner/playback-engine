use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::foundation::types::constants;

/// PCAP库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
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

impl Default for Configuration {
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

impl Configuration {
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



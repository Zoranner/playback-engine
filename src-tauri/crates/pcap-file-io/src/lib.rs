//! # PcapFile.IO - 高性能PCAP文件读写库
//!
//! 这是一个用Rust编写的高性能PCAP文件处理库，提供了完整的PCAP文件读写功能。
//!
//! ## 特性
//!
//! - 🚀 **高性能**: 零拷贝操作和编译时优化
//! - 🔒 **内存安全**: Rust的内存安全保证
//! - 🧵 **线程安全**: 内置线程安全支持
//! - 📦 **易于使用**: 简洁的API设计
//! - 🔧 **可配置**: 灵活的配置选项
//! - 📊 **完整功能**: 支持所有PCAP格式特性
//!
//! ## 快速开始
//!
//! ```rust
//! use pcap_file_io::{
//!     config::PcapConfiguration,
//!     structures::DataPacket,
//!     io::{PcapFileReader, PcapFileWriter},
//!     error::Result,
//! };
//!
//! fn main() -> Result<()> {
//!     // 创建配置
//!     let config = PcapConfiguration::default();
//!
//!     // 写入PCAP文件
//!     let mut writer = PcapFileWriter::new(config.clone());
//!     writer.create("example.pcap")?;
//!
//!     let data = b"Hello, World!".to_vec();
//!     let packet = DataPacket::from_datetime(
//!         std::time::SystemTime::now(),
//!         data,
//!     )?;
//!
//!     writer.write_packet(&packet)?;
//!     writer.close();
//!
//!     // 读取PCAP文件
//!     let mut reader = PcapFileReader::new(config);
//!     reader.open("example.pcap")?;
//!
//!     while let Some(packet) = reader.read_packet()? {
//!         println!("读取数据包: {:?}", packet);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 模块结构
//!
//! - `config`: 配置管理和常量定义
//! - `structures`: 数据结构和类型定义
//! - `utils`: 工具函数和扩展方法
//! - `io`: 文件读写操作
//! - `error`: 错误处理和结果类型
//!
//! ## 许可证
//!
//! MIT License

// 模块声明
pub mod config;
pub mod structures;
pub mod utils;
pub mod io;
pub mod error;

// 重新导出主要类型和功能
pub use config::{PcapConfiguration, PcapErrorCode};
pub use structures::{DataPacket, DataPacketHeader, PcapFileHeader};
pub use utils::{FileInfoCache, calculate_crc32, ByteArrayExtensions, DateTimeExtensions};
pub use io::{PcapFileReader, PcapFileWriter, MultiPcapReader};
pub use error::{PcapError, ErrorResult, Result};

// 版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// 获取库版本信息
pub fn version_info() -> &'static str {
    VERSION
}

/// 获取库作者信息
pub fn authors_info() -> &'static str {
    AUTHORS
}

/// 获取库描述信息
pub fn description_info() -> &'static str {
    DESCRIPTION
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_version_info() {
        assert!(!version_info().is_empty());
    }

    #[test]
    fn test_authors_info() {
        assert!(!authors_info().is_empty());
    }

    #[test]
    fn test_description_info() {
        assert!(!description_info().is_empty());
    }

    #[test]
    fn test_basic_functionality() {
        let config = PcapConfiguration::default();
        assert!(config.validate().is_ok());

        let data = b"Hello, World!".to_vec();
        let packet = DataPacket::from_datetime(SystemTime::now(), data.clone());
        assert!(packet.is_ok());

        let packet = packet.unwrap();
        assert_eq!(packet.packet_length(), data.len());
        assert!(packet.is_valid());
    }
}

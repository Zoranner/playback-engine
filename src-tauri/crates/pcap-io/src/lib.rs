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
//! - 📋 **索引支持**: 高性能PIDX索引文件处理
//!
//! ## 快速开始
//!
//! ```rust
//! use pcap_io::{
//!     config::Configuration,
//!     structures::DataPacket,
//!     file_reader::PcapFileReader,
//!     file_writer::PcapFileWriter,
//!     index::{PidxReader, PidxWriter},
//!     error::Result,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // 创建配置
//!     let config = Configuration::default();
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
//!     // 生成索引
//!     let index = PidxWriter::generate_index("data_directory").await?;
//!     PidxWriter::save_index(&index, "data_directory/dataset.pidx")?;
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
//! - `file_reader`: 单个文件读取器（内部）
//! - `file_writer`: 单个文件写入器（内部）
//! - `index`: 索引文件处理
//! - `error`: 错误处理和结果类型
//!
//! ## 许可证
//!
//! MIT License

// 模块声明
pub mod config;
pub mod error;
pub mod file_reader;
pub mod file_writer;
pub mod index;
pub mod reader;
pub mod structures;
pub mod traits;
pub mod utils;
pub mod writer;

// 重新导出主要类型和功能
pub use config::{Configuration, PcapErrorCode};
pub use error::{PcapError, Result};
pub use file_reader::PcapFileReader;
pub use file_writer::PcapFileWriter;
pub use index::{PacketIndexEntry, PcapFileIndex, PidxIndex, PidxReader, PidxWriter};
pub use reader::Reader;
pub use structures::{DataPacket, DataPacketHeader, DatasetInfo, FileInfo, PcapFileHeader};
pub use traits::{Info, Read, Write};
pub use utils::{calculate_crc32, ByteArrayExtensions, DateTimeExtensions, FileInfoCache};
pub use writer::Writer;

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

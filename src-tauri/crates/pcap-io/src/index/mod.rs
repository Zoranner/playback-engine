//! Index模块 - 高性能PCAP索引文件处理
//!
//! 提供PCAP文件的索引生成、读取和管理功能，支持快速时间戳查找和范围查询。

pub mod reader;
pub mod types;
pub mod writer;

// 重新导出主要类型
pub use reader::PidxReader;
pub use writer::PidxWriter;

// 重新导出数据结构
pub use types::{PacketIndexEntry, PcapFileIndex, PidxIndex};

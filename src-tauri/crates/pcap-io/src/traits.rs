//! Trait定义模块
//!
//! 定义了pcap-io库的核心trait接口，提供统一的读写和信息查询接口。

use crate::error::Result;
use crate::structures::{DataPacket, DatasetInfo, FileInfo};

/// 数据包读取trait
///
/// 提供数据包读取的标准接口，支持单个和批量读取操作。
pub trait Read {
    /// 读取下一个数据包
    ///
    /// # 返回
    /// - `Ok(Some(packet))` - 成功读取到数据包
    /// - `Ok(None)` - 到达文件末尾，无更多数据包
    /// - `Err(error)` - 读取过程中发生错误
    fn read_packet(&mut self) -> Result<Option<DataPacket>>;

    /// 批量读取多个数据包
    ///
    /// # 参数
    /// - `count` - 要读取的数据包数量
    ///
    /// # 返回
    /// 返回实际读取到的数据包列表，可能少于请求的数量
    fn read_packets(&mut self, count: usize) -> Result<Vec<DataPacket>>;

    /// 重置读取位置到开始
    ///
    /// 将读取器重置到数据集的开始位置，后续读取将从第一个数据包开始。
    fn reset(&mut self) -> Result<()>;
}

/// 数据包写入trait
///
/// 提供数据包写入的标准接口，支持单个和批量写入操作。
pub trait Write {
    /// 写入单个数据包
    ///
    /// # 参数
    /// - `packet` - 要写入的数据包
    fn write_packet(&mut self, packet: &DataPacket) -> Result<()>;

    /// 批量写入多个数据包
    ///
    /// # 参数
    /// - `packets` - 要写入的数据包切片
    fn write_packets(&mut self, packets: &[DataPacket]) -> Result<()>;

    /// 刷新缓冲区
    ///
    /// 强制将缓冲区中的数据写入到存储设备，确保数据持久化。
    fn flush(&mut self) -> Result<()>;
}

/// 数据集信息trait
///
/// 提供数据集统计信息和元数据的查询接口。
pub trait Info {
    /// 获取详细的数据集信息
    ///
    /// 包含数据包总数、文件总数、总大小、时间范围等完整信息
    fn dataset_info(&self) -> DatasetInfo;

    /// 获取详细文件列表
    ///
    /// 返回数据集中所有文件的详细信息，按需使用
    fn detailed_file_list(&self) -> Vec<FileInfo>;
}

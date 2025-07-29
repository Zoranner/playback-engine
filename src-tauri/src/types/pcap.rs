// 重新导出 pcapfile-io 库的类型，并添加应用特定的扩展
pub use pcapfile_io::{DataPacket, DataPacketHeader as PcapPacketHeader, PcapFileHeader};

use serde::{Deserialize, Serialize};

/// 数据包类型枚举 - 应用特定的分类
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PacketType {
    Environment,  // 环境信息
    Event,        // 事件信息
    Target,       // 目标信息
    Unknown,      // 未知类型
}

/// 应用级数据包结构 - 对 pcapfile-io 的 DataPacket 的包装
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDataPacket {
    /// 基础数据包信息
    pub base_packet: DataPacket,
    /// 数据包类型分类
    pub packet_type: PacketType,
}

impl AppDataPacket {
    /// 创建新的应用数据包
    pub fn new(base_packet: DataPacket, packet_type: PacketType) -> Self {
        Self {
            base_packet,
            packet_type,
        }
    }

    /// 获取时间戳（纳秒）
    pub fn get_timestamp_ns(&self) -> u64 {
        // 使用 pcapfile-io 的时间戳方法
        self.base_packet.get_timestamp_ns()
    }

    /// 获取数据内容
    pub fn get_data(&self) -> &[u8] {
        &self.base_packet.data
    }

    /// 获取数据大小
    pub fn get_size(&self) -> usize {
        self.base_packet.data.len()
    }
}

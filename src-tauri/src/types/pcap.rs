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

/// PCAP文件头部结构（16字节）
#[derive(Debug)]
pub struct PcapFileHeader {
    pub magic: u32,           // 魔数：0xD4C3B2A1
    pub major_version: u16,   // 主版本号：0x0002
    pub minor_version: u16,   // 次版本号：0x0004
    pub timezone_offset: u32, // 时区偏移量：通常为0
    pub timestamp_accuracy: u32, // 时间戳精度：固定为0
}

/// PCAP数据包头部结构（16字节）
#[derive(Debug)]
pub struct PcapPacketHeader {
    pub timestamp_sec: u32,      // 时间戳秒部分
    pub timestamp_nsec: u32,     // 时间戳纳秒部分
    pub packet_length: u32,      // 数据包长度
    pub checksum: u32,           // CRC32校验和
}

/// PCAP文件头部魔数
pub const PCAP_MAGIC_NUMBER: u32 = 0xD4C3B2A1;
/// PCAP协议主版本号
pub const PCAP_MAJOR_VERSION: u16 = 0x0002;
/// PCAP协议次版本号
pub const PCAP_MINOR_VERSION: u16 = 0x0004;

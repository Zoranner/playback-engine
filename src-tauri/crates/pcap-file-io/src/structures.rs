use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::config::constants;

/// PCAP文件头结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PcapFileHeader {
    /// 魔术数，固定值 0xD4C3B2A1
    pub magic_number: u32,
    /// 主版本号，固定值 0x0002
    pub major_version: u16,
    /// 次版本号，固定值 0x0004
    pub minor_version: u16,
    /// 时区偏移量（秒）
    pub timezone_offset: i32,
    /// 时间戳精度（纳秒）
    pub timestamp_accuracy: u32,
}

impl PcapFileHeader {
    /// 头部大小（字节）
    pub const HEADER_SIZE: usize = 16; // 4 + 2 + 2 + 4 + 4

    /// 默认时间戳精度（纳秒）
    pub const DEFAULT_TIMESTAMP_ACCURACY: u32 = 1;

    /// 创建新的PCAP文件头
    pub fn new(timezone_offset: i32) -> Self {
        Self {
            magic_number: constants::PCAP_MAGIC_NUMBER,
            major_version: constants::MAJOR_VERSION,
            minor_version: constants::MINOR_VERSION,
            timezone_offset,
            timestamp_accuracy: Self::DEFAULT_TIMESTAMP_ACCURACY,
        }
    }

    /// 从字节数组创建文件头
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < Self::HEADER_SIZE {
            return Err("字节数组长度不足".to_string());
        }

        let magic_number = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let major_version = u16::from_le_bytes([bytes[4], bytes[5]]);
        let minor_version = u16::from_le_bytes([bytes[6], bytes[7]]);
        let timezone_offset = i32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let timestamp_accuracy = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);

        Ok(Self {
            magic_number,
            major_version,
            minor_version,
            timezone_offset,
            timestamp_accuracy,
        })
    }

    /// 转换为字节数组
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::HEADER_SIZE);
        bytes.extend_from_slice(&self.magic_number.to_le_bytes());
        bytes.extend_from_slice(&self.major_version.to_le_bytes());
        bytes.extend_from_slice(&self.minor_version.to_le_bytes());
        bytes.extend_from_slice(&self.timezone_offset.to_le_bytes());
        bytes.extend_from_slice(&self.timestamp_accuracy.to_le_bytes());
        bytes
    }

    /// 验证文件头是否有效
    pub fn is_valid(&self) -> bool {
        self.magic_number == constants::PCAP_MAGIC_NUMBER
            && self.major_version == constants::MAJOR_VERSION
            && self.minor_version == constants::MINOR_VERSION
    }
}

/// 数据包头部结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPacketHeader {
    /// 时间戳（秒）
    pub timestamp_seconds: u32,
    /// 时间戳（纳秒）
    pub timestamp_nanoseconds: u32,
    /// 数据包长度
    pub packet_length: u32,
    /// 校验和
    pub checksum: u32,
}

impl DataPacketHeader {
    /// 头部大小（字节）
    pub const HEADER_SIZE: usize = 16; // 4 + 4 + 4 + 4

    /// 创建新的数据包头部
    pub fn new(
        timestamp_seconds: u32,
        timestamp_nanoseconds: u32,
        packet_length: u32,
        checksum: u32,
    ) -> Result<Self, String> {
        if !DataPacket::is_valid_size(packet_length as usize) {
            return Err(format!("无效的数据包长度: {}", packet_length));
        }

        Ok(Self {
            timestamp_seconds,
            timestamp_nanoseconds,
            packet_length,
            checksum,
        })
    }

    /// 从DateTime创建数据包头部
    pub fn from_datetime(capture_time: SystemTime, packet_length: u32, checksum: u32) -> Result<Self, String> {
        let duration = capture_time.duration_since(UNIX_EPOCH)
            .map_err(|_| "时间戳无效")?;

        let timestamp_seconds = duration.as_secs() as u32;
        let timestamp_nanoseconds = duration.subsec_nanos();

        Self::new(timestamp_seconds, timestamp_nanoseconds, packet_length, checksum)
    }

    /// 从数据包数据创建头部
    pub fn from_packet_data(capture_time: SystemTime, packet_data: &[u8]) -> Result<Self, String> {
        let checksum = crate::utils::calculate_crc32(packet_data);
        let packet_length = packet_data.len() as u32;

        Self::from_datetime(capture_time, packet_length, checksum)
    }

    /// 从字节数组创建头部
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < Self::HEADER_SIZE {
            return Err("字节数组长度不足".to_string());
        }

        let timestamp_seconds = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let timestamp_nanoseconds = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let packet_length = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let checksum = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);

        Self::new(timestamp_seconds, timestamp_nanoseconds, packet_length, checksum)
    }

    /// 转换为字节数组
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::HEADER_SIZE);
        bytes.extend_from_slice(&self.timestamp_seconds.to_le_bytes());
        bytes.extend_from_slice(&self.timestamp_nanoseconds.to_le_bytes());
        bytes.extend_from_slice(&self.packet_length.to_le_bytes());
        bytes.extend_from_slice(&self.checksum.to_le_bytes());
        bytes
    }

    /// 获取捕获时间
    pub fn capture_time(&self) -> SystemTime {
        UNIX_EPOCH + std::time::Duration::new(
            self.timestamp_seconds as u64,
            self.timestamp_nanoseconds,
        )
    }
}

/// 数据包结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPacket {
    /// 数据包头部
    pub header: DataPacketHeader,
    /// 数据包内容
    pub data: Vec<u8>,
}

impl DataPacket {
    /// 创建新的数据包
    pub fn new(header: DataPacketHeader, data: Vec<u8>) -> Result<Self, String> {
        if data.len() != header.packet_length as usize {
            return Err("数据长度与头部长度不匹配".to_string());
        }

        Ok(Self { header, data })
    }

    /// 从DateTime和数据创建数据包
    pub fn from_datetime(capture_time: SystemTime, data: Vec<u8>) -> Result<Self, String> {
        let header = DataPacketHeader::from_packet_data(capture_time, &data)?;
        Self::new(header, data)
    }

    /// 从时间戳和数据创建数据包
    pub fn from_timestamp(
        timestamp_seconds: u32,
        timestamp_nanoseconds: u32,
        data: Vec<u8>,
    ) -> Result<Self, String> {
        let checksum = crate::utils::calculate_crc32(&data);
        let packet_length = data.len() as u32;

        let header = DataPacketHeader::new(
            timestamp_seconds,
            timestamp_nanoseconds,
            packet_length,
            checksum,
        )?;

        Self::new(header, data)
    }

    /// 获取捕获时间
    pub fn capture_time(&self) -> SystemTime {
        self.header.capture_time()
    }

    /// 获取数据包长度
    pub fn packet_length(&self) -> usize {
        self.data.len()
    }

    /// 获取总大小（头部 + 数据）
    pub fn total_size(&self) -> usize {
        DataPacketHeader::HEADER_SIZE + self.packet_length()
    }

    /// 获取校验和
    pub fn checksum(&self) -> u32 {
        self.header.checksum
    }

    /// 验证数据包大小是否有效
    pub fn is_valid_size(size: usize) -> bool {
        size > 0 && size <= constants::MAX_PACKET_SIZE
    }

    /// 验证数据包是否有效
    pub fn is_valid(&self) -> bool {
        if !Self::is_valid_size(self.packet_length()) {
            return false;
        }

        let calculated_checksum = crate::utils::calculate_crc32(&self.data);
        calculated_checksum == self.header.checksum
    }

    /// 转换为字节数组（头部 + 数据）
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.total_size());
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.data);
        bytes
    }
}

impl std::fmt::Display for DataPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DataPacket {{ timestamp: {:?}, length: {}, checksum: 0x{:08X} }}",
            self.capture_time(),
            self.packet_length(),
            self.checksum()
        )
    }
}

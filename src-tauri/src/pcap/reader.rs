use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use byteorder::{LittleEndian, ReadBytesExt};
use crc32fast::Hasher;
use log::{debug, warn};

use crate::types::{DataPacket, PacketType, PlaybackError, Result};
use crate::types::{PcapFileHeader, PcapPacketHeader, PCAP_MAGIC_NUMBER, PCAP_MAJOR_VERSION, PCAP_MINOR_VERSION};

/// PCAP文件读取器
pub struct PcapReader {
    file_path: PathBuf,
    reader: BufReader<File>,
    file_header: PcapFileHeader,
    current_position: u64,
    total_packets: u64,
    file_size: u64,
}

impl PcapReader {
    /// 创建新的PCAP读取器
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let path = file_path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        let file_size = file.metadata()?.len();
        let mut reader = BufReader::new(file);

        // 读取并验证文件头
        let file_header = Self::read_file_header(&mut reader)?;

        debug!("成功打开PCAP文件: {:?}", path);
        debug!("文件大小: {} 字节", file_size);
        debug!("文件头: {:?}", file_header);

        let mut pcap_reader = Self {
            file_path: path,
            reader,
            file_header,
            current_position: 16, // 文件头后的位置
            total_packets: 0,
            file_size,
        };

        // 统计数据包总数
        pcap_reader.count_total_packets()?;

        Ok(pcap_reader)
    }

    /// 读取文件头
    fn read_file_header(reader: &mut BufReader<File>) -> Result<PcapFileHeader> {
        let magic = reader.read_u32::<LittleEndian>()?;
        if magic != PCAP_MAGIC_NUMBER {
            return Err(PlaybackError::FormatError(
                format!("无效的魔数: 0x{:08X}, 期望: 0x{:08X}", magic, PCAP_MAGIC_NUMBER)
            ));
        }

        let major_version = reader.read_u16::<LittleEndian>()?;
        let minor_version = reader.read_u16::<LittleEndian>()?;

        // 验证版本号
        if major_version != PCAP_MAJOR_VERSION {
            return Err(PlaybackError::FormatError(
                format!("不支持的主版本号: 0x{:04X}, 期望: 0x{:04X}", major_version, PCAP_MAJOR_VERSION)
            ));
        }

        if minor_version != PCAP_MINOR_VERSION {
            return Err(PlaybackError::FormatError(
                format!("不支持的次版本号: 0x{:04X}, 期望: 0x{:04X}", minor_version, PCAP_MINOR_VERSION)
            ));
        }

        let timezone_offset = reader.read_u32::<LittleEndian>()?;
        let timestamp_accuracy = reader.read_u32::<LittleEndian>()?;

        Ok(PcapFileHeader {
            magic,
            major_version,
            minor_version,
            timezone_offset,
            timestamp_accuracy,
        })
    }

    /// 统计数据包总数
    fn count_total_packets(&mut self) -> Result<()> {
        let original_position = self.current_position;
        let mut count = 0u64;

        // 跳转到数据包开始位置
        self.reader.seek(SeekFrom::Start(16))?;

        while self.reader.stream_position()? < self.file_size {
            match self.read_packet_header() {
                Ok(header) => {
                    count += 1;
                    // 跳过数据内容
                    self.reader.seek(SeekFrom::Current(header.packet_length as i64))?;
                }
                Err(_) => break, // 遇到错误时停止计数
            }
        }

        self.total_packets = count;

        // 恢复原始位置
        self.reader.seek(SeekFrom::Start(original_position))?;
        self.current_position = original_position;

        debug!("文件包含 {} 个数据包", self.total_packets);
        Ok(())
    }

    /// 读取数据包头部
    fn read_packet_header(&mut self) -> Result<PcapPacketHeader> {
        let timestamp_sec = self.reader.read_u32::<LittleEndian>()?;
        let timestamp_nsec = self.reader.read_u32::<LittleEndian>()?;
        let packet_length = self.reader.read_u32::<LittleEndian>()?;
        let checksum = self.reader.read_u32::<LittleEndian>()?;

        // 验证数据包长度
        if packet_length > 10 * 1024 * 1024 { // 10MB限制
            return Err(PlaybackError::FormatError(
                format!("数据包长度过大: {} 字节", packet_length)
            ));
        }

        Ok(PcapPacketHeader {
            timestamp_sec,
            timestamp_nsec,
            packet_length,
            checksum,
        })
    }

    /// 读取下一个数据包
    pub fn read_next_packet(&mut self) -> Result<Option<DataPacket>> {
        if self.reader.stream_position()? >= self.file_size {
            return Ok(None); // 文件结束
        }

        // 读取数据包头部
        let header = match self.read_packet_header() {
            Ok(h) => h,
            Err(e) => {
                warn!("读取数据包头部失败: {}", e);
                return Ok(None);
            }
        };

        // 读取数据内容
        let mut data = vec![0u8; header.packet_length as usize];
        self.reader.read_exact(&mut data)?;

        // 验证校验和
        if !self.verify_checksum(&data, header.checksum) {
            warn!("数据包校验和验证失败");
            // 继续处理，但记录警告
        }

        // 解析数据包类型
        let packet_type = self.parse_packet_type(&data);

        // 更新位置
        self.current_position = self.reader.stream_position()?;

        let packet = DataPacket::new(
            header.timestamp_sec,
            header.timestamp_nsec,
            data,
            packet_type,
        );

        debug!("读取数据包: 时间戳={}s {}ns, 大小={} 字节, 类型={:?}",
               packet.timestamp_sec, packet.timestamp_nsec, packet.size, packet.packet_type);

        Ok(Some(packet))
    }

    /// 验证CRC32校验和
    fn verify_checksum(&self, data: &[u8], expected_checksum: u32) -> bool {
        let mut hasher = Hasher::new();
        hasher.update(data);
        let actual_checksum = hasher.finalize();
        actual_checksum == expected_checksum
    }

    /// 解析数据包类型（简单实现，可根据实际数据格式扩展）
    fn parse_packet_type(&self, data: &[u8]) -> PacketType {
        if data.is_empty() {
            return PacketType::Unknown;
        }

        // 简单的类型识别逻辑（可根据实际需求扩展）
        // 这里假设数据包的第一个字节表示类型
        match data.get(0) {
            Some(0x01) => PacketType::Environment,
            Some(0x02) => PacketType::Event,
            Some(0x03) => PacketType::Target,
            _ => PacketType::Unknown,
        }
    }

    /// 跳转到指定时间点
    pub fn seek_to_time(&mut self, target_time: u64) -> Result<()> {
        debug!("跳转到时间点: {} ns", target_time);

        // 重置到文件开头的数据包位置
        self.reader.seek(SeekFrom::Start(16))?;
        self.current_position = 16;

        // 线性搜索指定时间点（可优化为二分搜索）
        while self.reader.stream_position()? < self.file_size {
            let position_before_header = self.reader.stream_position()?;

            match self.read_packet_header() {
                Ok(header) => {
                    let packet_time = (header.timestamp_sec as u64) * 1_000_000_000 + (header.timestamp_nsec as u64);

                    if packet_time >= target_time {
                        // 找到目标时间点，回退到数据包头部
                        self.reader.seek(SeekFrom::Start(position_before_header))?;
                        self.current_position = position_before_header;
                        debug!("成功跳转到时间点: {} ns", target_time);
                        return Ok(());
                    }

                    // 跳过数据内容
                    self.reader.seek(SeekFrom::Current(header.packet_length as i64))?;
                }
                Err(_) => break,
            }
        }

        warn!("未找到指定时间点: {} ns", target_time);
        Ok(())
    }

    /// 获取文件总时长（纳秒）
    pub fn get_total_duration(&mut self) -> Result<u64> {
        if self.total_packets == 0 {
            return Ok(0);
        }

        let original_position = self.current_position;

        // 读取第一个数据包的时间戳
        self.reader.seek(SeekFrom::Start(16))?;
        let first_header = self.read_packet_header()?;
        let first_time = (first_header.timestamp_sec as u64) * 1_000_000_000 + (first_header.timestamp_nsec as u64);

        // 读取最后一个数据包的时间戳
        // 这里使用简单的方法，从后往前搜索
        let mut last_time = first_time;
        self.reader.seek(SeekFrom::Start(16))?;

        while self.reader.stream_position()? < self.file_size {
            match self.read_packet_header() {
                Ok(header) => {
                    last_time = (header.timestamp_sec as u64) * 1_000_000_000 + (header.timestamp_nsec as u64);
                    self.reader.seek(SeekFrom::Current(header.packet_length as i64))?;
                }
                Err(_) => break,
            }
        }

        // 恢复原始位置
        self.reader.seek(SeekFrom::Start(original_position))?;
        self.current_position = original_position;

        let duration = last_time - first_time;
        debug!("文件总时长: {} ns ({:.2} 秒)", duration, duration as f64 / 1_000_000_000.0);

        Ok(duration)
    }

    /// 获取文件路径
    pub fn get_file_path(&self) -> &Path {
        &self.file_path
    }

    /// 获取数据包总数
    pub fn get_total_packets(&self) -> u64 {
        self.total_packets
    }

    /// 获取当前读取位置
    pub fn get_current_position(&self) -> u64 {
        self.current_position
    }

    /// 重置到文件开头
    pub fn reset(&mut self) -> Result<()> {
        self.reader.seek(SeekFrom::Start(16))?;
        self.current_position = 16;
        debug!("PCAP读取器已重置到文件开头");
        Ok(())
    }

    /// 跳转到指定字节位置
    pub fn seek_to_byte_position(&mut self, position: u64) -> Result<()> {
        if position < 16 {
            return Err(PlaybackError::FormatError(
                "字节位置不能小于文件头大小（16字节）".to_string()
            ));
        }

        if position >= self.file_size {
            return Err(PlaybackError::FormatError(
                format!("字节位置超出文件大小: {} >= {}", position, self.file_size)
            ));
        }

        self.reader.seek(SeekFrom::Start(position))?;
        self.current_position = position;
        debug!("跳转到字节位置: {}", position);
        Ok(())
    }
}

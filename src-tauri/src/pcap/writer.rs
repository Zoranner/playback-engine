use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use byteorder::{LittleEndian, WriteBytesExt};
use crc32fast::Hasher;
use log::{debug, info};

use crate::types::{DataPacket, PlaybackError, Result};
use crate::types::{PCAP_MAGIC_NUMBER, PCAP_MAJOR_VERSION, PCAP_MINOR_VERSION};

/// PCAP文件写入器
pub struct PcapWriter {
    file_path: PathBuf,
    writer: BufWriter<File>,
    packets_written: u64,
}

impl PcapWriter {
    /// 创建新的PCAP写入器
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let path = file_path.as_ref().to_path_buf();
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);

        // 写入文件头
        Self::write_file_header(&mut writer)?;

        info!("创建PCAP写入器: {:?}", path);

        Ok(Self {
            file_path: path,
            writer,
            packets_written: 0,
        })
    }

    /// 写入文件头
    fn write_file_header(writer: &mut BufWriter<File>) -> Result<()> {
        writer.write_u32::<LittleEndian>(PCAP_MAGIC_NUMBER)?;
        writer.write_u16::<LittleEndian>(PCAP_MAJOR_VERSION)?;
        writer.write_u16::<LittleEndian>(PCAP_MINOR_VERSION)?;
        writer.write_u32::<LittleEndian>(0)?; // timezone_offset
        writer.write_u32::<LittleEndian>(0)?; // timestamp_accuracy
        Ok(())
    }

    /// 写入数据包
    pub fn write_packet(&mut self, packet: &DataPacket) -> Result<()> {
        // 计算校验和
        let checksum = self.calculate_checksum(&packet.data);

        // 写入数据包头部
        self.writer.write_u32::<LittleEndian>(packet.timestamp_sec)?;
        self.writer.write_u32::<LittleEndian>(packet.timestamp_nsec)?;
        self.writer.write_u32::<LittleEndian>(packet.data.len() as u32)?;
        self.writer.write_u32::<LittleEndian>(checksum)?;

        // 写入数据内容
        self.writer.write_all(&packet.data)?;

        self.packets_written += 1;

        debug!("写入数据包: 时间戳={}s {}ns, 大小={} 字节",
               packet.timestamp_sec, packet.timestamp_nsec, packet.data.len());

        Ok(())
    }

    /// 批量写入数据包
    pub fn write_packets(&mut self, packets: &[DataPacket]) -> Result<()> {
        for packet in packets {
            self.write_packet(packet)?;
        }
        Ok(())
    }

    /// 计算CRC32校验和
    fn calculate_checksum(&self, data: &[u8]) -> u32 {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize()
    }

    /// 刷新缓冲区
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }

    /// 获取已写入的数据包数量
    pub fn get_packets_written(&self) -> u64 {
        self.packets_written
    }

    /// 获取文件路径
    pub fn get_file_path(&self) -> &Path {
        &self.file_path
    }
}

impl Drop for PcapWriter {
    fn drop(&mut self) {
        if let Err(e) = self.flush() {
            log::error!("刷新PCAP写入器时出错: {}", e);
        }
        info!("PCAP写入器已关闭: {:?}, 共写入 {} 个数据包",
              self.file_path, self.packets_written);
    }
}

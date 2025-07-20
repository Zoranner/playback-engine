use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use log::{info, warn};

use crate::config::PcapConfiguration;
use crate::structures::{DataPacket, DataPacketHeader, PcapFileHeader};
use crate::utils::{FileInfoCache, calculate_crc32};

/// PCAP文件读取器
pub struct PcapFileReader {
    file: Option<File>,
    reader: Option<BufReader<File>>,
    file_path: Option<PathBuf>,
    packet_count: u64,
    file_size: u64,
    header: Option<PcapFileHeader>,
    header_position: u64,
    configuration: PcapConfiguration,
}

impl PcapFileReader {
    pub fn new(configuration: PcapConfiguration) -> Self {
        Self {
            file: None,
            reader: None,
            file_path: None,
            packet_count: 0,
            file_size: 0,
            header: None,
            header_position: 0,
            configuration,
        }
    }

    /// 打开PCAP文件
    pub fn open<P: AsRef<Path>>(&mut self, file_path: P) -> Result<(), String> {
        let path = file_path.as_ref();

        if !path.exists() {
            return Err(format!("文件不存在: {:?}", path));
        }

        let file = File::open(path)
            .map_err(|e| format!("无法打开文件: {:?}, 错误: {}", path, e))?;

        let file_size = file.metadata()
            .map_err(|e| format!("无法获取文件元数据: {}", e))?
            .len();

        if file_size < PcapFileHeader::HEADER_SIZE as u64 {
            return Err("文件太小，不是有效的PCAP文件".to_string());
        }

        let mut reader = BufReader::new(file);

        // 读取并验证文件头
        let header = self.read_and_validate_header(&mut reader)?;

        self.file = Some(reader.get_ref().try_clone()
            .map_err(|e| format!("无法克隆文件句柄: {}", e))?);
        self.reader = Some(reader);
        self.file_path = Some(path.to_path_buf());
        self.file_size = file_size;
        self.header = Some(header);
        self.packet_count = 0;

        info!("成功打开PCAP文件: {:?}", path);
        Ok(())
    }

    /// 读取并验证文件头
    fn read_and_validate_header(&self, reader: &mut BufReader<File>) -> Result<PcapFileHeader, String> {
        let mut header_bytes = [0u8; PcapFileHeader::HEADER_SIZE];
        reader.read_exact(&mut header_bytes)
            .map_err(|e| format!("读取文件头失败: {}", e))?;

        let header = PcapFileHeader::from_bytes(&header_bytes)?;

        if !header.is_valid() {
            return Err("无效的PCAP文件头".to_string());
        }

        Ok(header)
    }

    /// 读取下一个数据包
    pub fn read_packet(&mut self) -> Result<Option<DataPacket>, String> {
        let reader = self.reader.as_mut()
            .ok_or("文件未打开")?;

        // 读取数据包头部
        let mut header_bytes = [0u8; DataPacketHeader::HEADER_SIZE];
        match reader.read_exact(&mut header_bytes) {
            Ok(_) => {},
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                return Ok(None); // 到达文件末尾
            },
            Err(e) => return Err(format!("读取数据包头部失败: {}", e)),
        }

        let header = DataPacketHeader::from_bytes(&header_bytes)?;

        // 读取数据包内容
        let mut data = vec![0u8; header.packet_length as usize];
        reader.read_exact(&mut data)
            .map_err(|e| format!("读取数据包内容失败: {}", e))?;

        // 验证校验和
        if self.configuration.enable_validation {
            let calculated_checksum = calculate_crc32(&data);
            if calculated_checksum != header.checksum {
                return Err(format!(
                    "数据包校验和验证失败。期望: 0x{:08X}, 实际: 0x{:08X}",
                    header.checksum, calculated_checksum
                ));
            }
        }

        self.packet_count += 1;
        Ok(Some(DataPacket::new(header, data)?))
    }

    /// 重置读取位置到数据区开始位置
    pub fn reset(&mut self) -> Result<(), String> {
        let reader = self.reader.as_mut()
            .ok_or("文件未打开")?;

        reader.seek(SeekFrom::Start(self.header_position + PcapFileHeader::HEADER_SIZE as u64))
            .map_err(|e| format!("重置读取位置失败: {}", e))?;

        self.packet_count = 0;
        Ok(())
    }

    /// 移动到指定的字节位置
    pub fn seek(&mut self, position: u64) -> Result<(), String> {
        let reader = self.reader.as_mut()
            .ok_or("文件未打开")?;

        let min_position = self.header_position + PcapFileHeader::HEADER_SIZE as u64;
        if position < min_position {
            return Err(format!("位置不能小于数据区开始位置: {}", min_position));
        }

        reader.seek(SeekFrom::Start(position))
            .map_err(|e| format!("移动到指定位置失败: {}", e))?;

        Ok(())
    }

    /// 关闭文件
    pub fn close(&mut self) {
        self.reader = None;
        self.file = None;
        self.file_path = None;
        self.packet_count = 0;
        self.file_size = 0;
        self.header = None;
    }

    /// 获取当前文件路径
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    /// 获取文件大小
    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    /// 获取已读取的数据包数量
    pub fn packet_count(&self) -> u64 {
        self.packet_count
    }

    /// 获取文件头
    pub fn header(&self) -> Option<&PcapFileHeader> {
        self.header.as_ref()
    }

    /// 检查是否已到达文件末尾
    pub fn is_eof(&self) -> bool {
        if let Some(reader) = &self.reader {
            reader.get_ref().metadata()
                .map(|m| reader.get_ref().stream_position().unwrap_or(0) >= m.len())
                .unwrap_or(true)
        } else {
            true
        }
    }
}

/// PCAP文件写入器
pub struct PcapFileWriter {
    file: Option<File>,
    writer: Option<BufWriter<File>>,
    file_path: Option<PathBuf>,
    packet_count: u64,
    total_size: u64,
    max_packets_per_file: usize,
    configuration: PcapConfiguration,
}

impl PcapFileWriter {
    pub fn new(configuration: PcapConfiguration) -> Self {
        Self {
            file: None,
            writer: None,
            file_path: None,
            packet_count: 0,
            total_size: 0,
            max_packets_per_file: configuration.max_packets_per_file,
            configuration,
        }
    }

    /// 创建新的PCAP文件
    pub fn create<P: AsRef<Path>>(&mut self, file_path: P) -> Result<(), String> {
        let path = file_path.as_ref();

        // 确保目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("创建目录失败: {}", e))?;
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)
            .map_err(|e| format!("创建文件失败: {:?}, 错误: {}", path, e))?;

        let mut writer = BufWriter::with_capacity(self.configuration.buffer_size, file);

        // 写入文件头
        let header = PcapFileHeader::new(0);
        writer.write_all(&header.to_bytes())
            .map_err(|e| format!("写入文件头失败: {}", e))?;

        if self.configuration.auto_flush {
            writer.flush()
                .map_err(|e| format!("刷新缓冲区失败: {}", e))?;
        }

        self.file = Some(writer.get_ref().try_clone()
            .map_err(|e| format!("无法克隆文件句柄: {}", e))?);
        self.writer = Some(writer);
        self.file_path = Some(path.to_path_buf());
        self.packet_count = 0;
        self.total_size = PcapFileHeader::HEADER_SIZE as u64;

        info!("成功创建PCAP文件: {:?}", path);
        Ok(())
    }

    /// 写入数据包
    pub fn write_packet(&mut self, packet: &DataPacket) -> Result<u64, String> {
        // 检查是否需要创建新文件
        if self.packet_count >= self.max_packets_per_file as u64 {
            self.create_new_file()?;
        }

        let writer = self.writer.as_mut()
            .ok_or("文件未打开")?;

        // 获取当前位置作为偏移量
        let offset = self.total_size;

        // 写入数据包
        let packet_bytes = packet.to_bytes();
        writer.write_all(&packet_bytes)
            .map_err(|e| format!("写入数据包失败: {}", e))?;

        self.packet_count += 1;
        self.total_size += packet_bytes.len() as u64;

        if self.configuration.auto_flush {
            writer.flush()
                .map_err(|e| format!("刷新缓冲区失败: {}", e))?;
        }

        Ok(offset)
    }

    /// 创建新文件
    fn create_new_file(&mut self) -> Result<(), String> {
        let current_path = self.file_path.clone();
        if let Some(path) = current_path {
            // 关闭当前文件
            self.close();

            // 生成新文件名
            let new_path = self.generate_new_file_path(&path)?;

            // 创建新文件
            self.create(new_path)?;
        }
        Ok(())
    }

    /// 生成新文件路径
    fn generate_new_file_path(&self, current_path: &Path) -> Result<PathBuf, String> {
        let stem = current_path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or("无法获取文件名")?;

        let extension = current_path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("pcap");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "获取时间戳失败")?
            .as_nanos();

        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        Ok(current_path.with_file_name(new_filename))
    }

    /// 刷新缓冲区
    pub fn flush(&mut self) -> Result<(), String> {
        if let Some(writer) = &mut self.writer {
            writer.flush()
                .map_err(|e| format!("刷新缓冲区失败: {}", e))?;
        }
        Ok(())
    }

    /// 关闭文件
    pub fn close(&mut self) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.flush();
        }
        self.writer = None;
        self.file = None;
        self.file_path = None;
        self.packet_count = 0;
        self.total_size = 0;
    }

    /// 获取当前文件路径
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    /// 获取已写入的数据包数量
    pub fn packet_count(&self) -> u64 {
        self.packet_count
    }

    /// 获取总大小
    pub fn total_size(&self) -> u64 {
        self.total_size
    }
}

/// 多文件PCAP读取器
pub struct MultiPcapReader {
    files: Vec<PathBuf>,
    current_file_index: usize,
    current_reader: Option<PcapFileReader>,
    total_packet_count: u64,
    configuration: PcapConfiguration,
    file_cache: Arc<Mutex<FileInfoCache>>,
}

impl MultiPcapReader {
    pub fn new<P: AsRef<Path>>(directory: P, configuration: PcapConfiguration) -> Result<Self, String> {
        let dir = directory.as_ref();
        if !dir.exists() || !dir.is_dir() {
            return Err(format!("目录不存在或不是目录: {:?}", dir));
        }

        // 扫描目录中的PCAP文件
        let mut files = Vec::new();
        for entry in std::fs::read_dir(dir)
            .map_err(|e| format!("读取目录失败: {}", e))? {
            let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("pcap") {
                files.push(path);
            }
        }

        if files.is_empty() {
            return Err("目录中没有找到PCAP文件".to_string());
        }

        // 按文件名排序
        files.sort();

        let file_cache = Arc::new(Mutex::new(FileInfoCache::new(configuration.index_cache_size)));

        Ok(Self {
            files,
            current_file_index: 0,
            current_reader: None,
            total_packet_count: 0,
            configuration,
            file_cache,
        })
    }

    /// 读取下一个数据包
    pub fn read_next_packet(&mut self) -> Result<Option<DataPacket>, String> {
        loop {
            // 如果当前读取器为空或已到达文件末尾，尝试打开下一个文件
            if self.current_reader.is_none() || self.current_reader.as_ref().unwrap().is_eof() {
                if !self.open_next_file()? {
                    return Ok(None); // 没有更多文件
                }
            }

            // 从当前文件读取数据包
            if let Some(reader) = &mut self.current_reader {
                match reader.read_packet()? {
                    Some(packet) => {
                        self.total_packet_count += 1;
                        return Ok(Some(packet));
                    }
                    None => {
                        // 当前文件已读完，继续下一个文件
                        continue;
                    }
                }
            }
        }
    }

    /// 打开下一个文件
    fn open_next_file(&mut self) -> Result<bool, String> {
        if self.current_file_index >= self.files.len() {
            return Ok(false); // 没有更多文件
        }

        let file_path = &self.files[self.current_file_index];
        let mut reader = PcapFileReader::new(self.configuration.clone());

        match reader.open(file_path) {
            Ok(_) => {
                self.current_reader = Some(reader);
                self.current_file_index += 1;
                info!("打开文件: {:?}", file_path);
                Ok(true)
            }
            Err(e) => {
                warn!("无法打开文件 {:?}: {}", file_path, e);
                self.current_file_index += 1;
                // 尝试下一个文件
                self.open_next_file()
            }
        }
    }

    /// 重置读取位置
    pub fn reset(&mut self) -> Result<(), String> {
        self.current_file_index = 0;
        self.current_reader = None;
        self.total_packet_count = 0;
        Ok(())
    }

    /// 获取文件列表
    pub fn get_files(&self) -> &[PathBuf] {
        &self.files
    }

    /// 获取总数据包数量
    pub fn get_total_packet_count(&self) -> u64 {
        self.total_packet_count
    }

    /// 获取缓存统计信息
    pub fn get_cache_statistics(&self) -> Result<crate::utils::CacheStatistics, String> {
        let cache = self.file_cache.lock()
            .map_err(|_| "缓存锁定失败")?;
        cache.get_statistics()
    }
}

impl Drop for PcapFileReader {
    fn drop(&mut self) {
        self.close();
    }
}

impl Drop for PcapFileWriter {
    fn drop(&mut self) {
        self.close();
    }
}

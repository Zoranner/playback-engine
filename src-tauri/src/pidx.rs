use std::collections::HashMap;
use std::fs::{File, self};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use log::{debug, info, warn};

use crate::types::{PlaybackError, Result};
use crate::pcap_reader::PcapReader;

/// 单个数据包在索引中的记录
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "packet")]
pub struct PacketIndexEntry {
    /// 时间戳（纳秒）
    pub timestamp_ns: u64,
    /// 所在文件名
    pub file_name: String,
    /// 在文件中的字节位置
    pub byte_offset: u64,
    /// 数据包大小
    pub packet_size: u32,
}

/// 单个PCAP文件的索引信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "file")]
pub struct PcapFileIndex {
    /// 文件名
    pub file_name: String,
    /// 文件SHA256哈希值
    pub file_hash: String,
    /// 文件大小（字节）
    pub file_size: u64,
    /// 数据包数量
    pub packet_count: u64,
    /// 文件中第一个数据包的时间戳
    pub start_timestamp: u64,
    /// 文件中最后一个数据包的时间戳
    pub end_timestamp: u64,
    /// 该文件中所有数据包的索引条目
    #[serde(rename = "packet")]
    pub packets: Vec<PacketIndexEntry>,
}

/// PIDX索引文件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "pidx_index")]
pub struct PidxIndex {
    /// 索引创建时间
    pub created_at: String,
    /// 索引版本
    pub version: String,
    /// 数据集名称
    pub dataset_name: String,
    /// 数据集路径
    pub dataset_path: String,
    /// 总数据包数量
    pub total_packets: u64,
    /// 总时长（纳秒）
    pub total_duration: u64,
    /// 开始时间戳
    pub start_timestamp: u64,
    /// 结束时间戳
    pub end_timestamp: u64,
    /// 所有PCAP文件的索引
    #[serde(rename = "file")]
    pub files: Vec<PcapFileIndex>,
    /// 时间戳到文件位置的快速查找映射
    #[serde(skip)]
    pub timestamp_index: HashMap<u64, PacketIndexEntry>,
}

impl PidxIndex {
    /// 创建新的空索引
    pub fn new(dataset_name: String, dataset_path: String) -> Self {
        Self {
            created_at: chrono::Utc::now().to_rfc3339(),
            version: "1.0".to_string(),
            dataset_name,
            dataset_path,
            total_packets: 0,
            total_duration: 0,
            start_timestamp: 0,
            end_timestamp: 0,
            files: Vec::new(),
            timestamp_index: HashMap::new(),
        }
    }

    /// 构建时间戳快速查找索引
    pub fn build_timestamp_index(&mut self) {
        self.timestamp_index.clear();

        for file_index in &self.files {
            for packet in &file_index.packets {
                self.timestamp_index.insert(packet.timestamp_ns, packet.clone());
            }
        }

        debug!("构建时间戳索引完成，包含 {} 个条目", self.timestamp_index.len());
    }

    /// 根据时间戳查找数据包位置
    pub fn find_packet_by_timestamp(&self, target_timestamp: u64) -> Option<&PacketIndexEntry> {
        // 首先尝试精确匹配
        if let Some(entry) = self.timestamp_index.get(&target_timestamp) {
            return Some(entry);
        }

        // 如果没有精确匹配，找到最接近的时间戳
        let mut closest_entry: Option<&PacketIndexEntry> = None;
        let mut min_diff = u64::MAX;

        for entry in self.timestamp_index.values() {
            let diff = if entry.timestamp_ns >= target_timestamp {
                entry.timestamp_ns - target_timestamp
            } else {
                target_timestamp - entry.timestamp_ns
            };

            if diff < min_diff {
                min_diff = diff;
                closest_entry = Some(entry);
            }
        }

        closest_entry
    }

    /// 获取指定时间范围内的所有数据包
    pub fn get_packets_in_range(&self, start_time: u64, end_time: u64) -> Vec<&PacketIndexEntry> {
        self.timestamp_index
            .values()
            .filter(|entry| entry.timestamp_ns >= start_time && entry.timestamp_ns <= end_time)
            .collect()
    }
}

/// PIDX文件读写器
pub struct PidxManager {
    index: Option<PidxIndex>,
}

impl PidxManager {
    /// 创建新的PIDX管理器
    pub fn new() -> Self {
        Self { index: None }
    }

    /// 计算文件的SHA256哈希值
    pub fn calculate_file_hash<P: AsRef<Path>>(file_path: P) -> Result<String> {
        let file = File::open(file_path.as_ref())?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }

    /// 验证PCAP文件是否与索引中的哈希值匹配
    pub fn verify_file_hash<P: AsRef<Path>>(file_path: P, expected_hash: &str) -> Result<bool> {
        let actual_hash = Self::calculate_file_hash(file_path)?;
        Ok(actual_hash == expected_hash)
    }

    /// 生成数据集的时间索引
    pub async fn generate_index<P: AsRef<Path>>(dataset_path: P) -> Result<PidxIndex> {
        let path = dataset_path.as_ref();
        let dataset_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("未命名数据集")
            .to_string();

        info!("开始生成数据集时间索引: {}", dataset_name);

        let mut index = PidxIndex::new(dataset_name, path.to_string_lossy().to_string());

        // 扫描目录中的所有PCAP文件
        let pcap_files = Self::scan_pcap_files(path)?;

        if pcap_files.is_empty() {
            return Err(PlaybackError::ProjectError(
                "数据集目录中未找到PCAP文件".to_string()
            ));
        }

        info!("找到 {} 个PCAP文件，开始分析...", pcap_files.len());

        let mut global_start_timestamp = u64::MAX;
        let mut global_end_timestamp = 0u64;

        // 分析每个PCAP文件
        for file_path in pcap_files {
            match Self::index_pcap_file(&file_path).await {
                Ok(file_index) => {
                    // 更新全局时间戳
                    if file_index.start_timestamp < global_start_timestamp {
                        global_start_timestamp = file_index.start_timestamp;
                    }
                    if file_index.end_timestamp > global_end_timestamp {
                        global_end_timestamp = file_index.end_timestamp;
                    }

                    // 更新总计数
                    index.total_packets += file_index.packet_count;

                    index.files.push(file_index);
                }
                Err(e) => {
                    warn!("分析PCAP文件失败: {:?}, 错误: {}", file_path, e);
                    // 继续处理其他文件
                }
            }
        }

        // 设置全局时间信息
        index.start_timestamp = global_start_timestamp;
        index.end_timestamp = global_end_timestamp;
        index.total_duration = global_end_timestamp - global_start_timestamp;

        // 构建时间戳快速查找索引
        index.build_timestamp_index();

        info!("索引生成完成 - 文件数: {}, 总数据包: {}, 时长: {:.2}秒",
              index.files.len(),
              index.total_packets,
              index.total_duration as f64 / 1_000_000_000.0);

        Ok(index)
    }

    /// 扫描目录中的PCAP文件
    fn scan_pcap_files<P: AsRef<Path>>(dir_path: P) -> Result<Vec<PathBuf>> {
        let mut pcap_files = Vec::new();
        let entries = fs::read_dir(dir_path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.to_str() == Some("pcap") {
                        pcap_files.push(path);
                    }
                }
            }
        }

        // 按文件名排序
        pcap_files.sort();
        Ok(pcap_files)
    }

    /// 为单个PCAP文件生成索引
    async fn index_pcap_file<P: AsRef<Path>>(file_path: P) -> Result<PcapFileIndex> {
        let path = file_path.as_ref();
        let file_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        debug!("正在分析PCAP文件: {}", file_name);

        // 计算文件哈希
        let file_hash = Self::calculate_file_hash(path)?;
        let file_size = fs::metadata(path)?.len();

        // 打开PCAP文件并读取所有数据包
        let mut reader = PcapReader::new(path)?;
        let mut packets = Vec::new();
        let mut packet_count = 0u64;
        let mut current_position = 16u64; // PCAP文件头后的位置

        let mut start_timestamp = u64::MAX;
        let mut end_timestamp = 0u64;

        // 读取所有数据包并记录位置
        while let Some(packet) = reader.read_next_packet()? {
            let timestamp_ns = packet.get_timestamp_ns();

            // 更新时间范围
            if timestamp_ns < start_timestamp {
                start_timestamp = timestamp_ns;
            }
            if timestamp_ns > end_timestamp {
                end_timestamp = timestamp_ns;
            }

            // 创建索引条目
            let index_entry = PacketIndexEntry {
                timestamp_ns,
                file_name: file_name.clone(),
                byte_offset: current_position,
                packet_size: packet.size,
            };

            packets.push(index_entry);
            packet_count += 1;

            // 更新当前位置（16字节包头 + 数据内容）
            current_position += 16 + packet.size as u64;
        }

        let file_index = PcapFileIndex {
            file_name,
            file_hash,
            file_size,
            packet_count,
            start_timestamp,
            end_timestamp,
            packets,
        };

        debug!("文件分析完成: {} 个数据包, 时间范围: {}ns - {}ns",
               packet_count, start_timestamp, end_timestamp);

        Ok(file_index)
    }

    /// 保存索引到PIDX文件
    pub fn save_index<P: AsRef<Path>>(index: &PidxIndex, pidx_file_path: P) -> Result<()> {
        let xml_content = Self::serialize_to_xml(index)?;
        fs::write(pidx_file_path.as_ref(), xml_content)?;

        info!("PIDX索引文件已保存: {:?}", pidx_file_path.as_ref());
        Ok(())
    }

    /// 从PIDX文件加载索引
    pub fn load_index<P: AsRef<Path>>(pidx_file_path: P) -> Result<PidxIndex> {
        let xml_content = fs::read_to_string(pidx_file_path.as_ref())?;
        let mut index = Self::deserialize_from_xml(&xml_content)?;

        // 重建时间戳索引
        index.build_timestamp_index();

        info!("PIDX索引文件已加载: {:?}", pidx_file_path.as_ref());
        Ok(index)
    }

    /// 将索引序列化为XML格式
    fn serialize_to_xml(index: &PidxIndex) -> Result<String> {
        let xml_string = serde_xml_rs::to_string(index)
            .map_err(|e| PlaybackError::FormatError(format!("XML序列化失败: {}", e)))?;

        // 添加XML声明
        let xml_with_declaration = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
{}"#, xml_string);

        Ok(xml_with_declaration)
    }

    /// 从XML格式反序列化索引
    fn deserialize_from_xml(xml_content: &str) -> Result<PidxIndex> {
        let mut index: PidxIndex = serde_xml_rs::from_str(xml_content)
            .map_err(|e| PlaybackError::FormatError(format!("XML反序列化失败: {}", e)))?;

        // 重建时间戳索引（因为它被跳过序列化）
        index.build_timestamp_index();

        Ok(index)
    }

    /// 验证索引文件的有效性
    pub async fn verify_index_validity<P: AsRef<Path>>(
        index: &PidxIndex,
        dataset_path: P
    ) -> Result<bool> {
        let path = dataset_path.as_ref();

        info!("验证索引文件有效性...");

        for file_index in &index.files {
            let file_path = path.join(&file_index.file_name);

            if !file_path.exists() {
                warn!("PCAP文件不存在: {:?}", file_path);
                return Ok(false);
            }

            // 验证文件哈希
            match Self::verify_file_hash(&file_path, &file_index.file_hash) {
                Ok(true) => {
                    debug!("文件哈希验证通过: {}", file_index.file_name);
                }
                Ok(false) => {
                    warn!("文件哈希验证失败: {}", file_index.file_name);
                    return Ok(false);
                }
                Err(e) => {
                    warn!("计算文件哈希失败: {}, 错误: {}", file_index.file_name, e);
                    return Ok(false);
                }
            }
        }

        info!("索引文件验证通过");
        Ok(true)
    }

    /// 获取当前加载的索引
    pub fn get_index(&self) -> Option<&PidxIndex> {
        self.index.as_ref()
    }

    /// 设置索引
    pub fn set_index(&mut self, index: PidxIndex) {
        self.index = Some(index);
    }
}

impl Default for PidxManager {
    fn default() -> Self {
        Self::new()
    }
}

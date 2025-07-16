use std::fs::{File, self};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use log::{info, warn, debug};
use serde::{Serialize, Deserialize};

use crate::types::{PlaybackError, Result, PidxIndex};

/// PIDX文件读取器
pub struct PidxReader;

impl PidxReader {
    /// 从PIDX文件加载索引
    pub fn load_index<P: AsRef<Path>>(pidx_file_path: P) -> Result<PidxIndex> {
        let xml_content = fs::read_to_string(pidx_file_path.as_ref())?;
        let mut index = Self::deserialize_from_xml(&xml_content)?;

        // 重建时间戳索引
        index.build_timestamp_index();

        info!("PIDX索引文件已加载: {:?}", pidx_file_path.as_ref());
        Ok(index)
    }

    /// 从数据集目录查找PIDX文件
    pub fn find_pidx_file<P: AsRef<Path>>(dataset_path: P) -> Result<Option<PathBuf>> {
        let entries = fs::read_dir(dataset_path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.to_str() == Some("pidx") {
                        return Ok(Some(path));
                    }
                }
            }
        }

        Ok(None)
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

    /// 从XML格式反序列化索引
    fn deserialize_from_xml(xml_content: &str) -> Result<PidxIndex> {
        let mut index: PidxIndex = serde_xml_rs::from_str(xml_content)
            .map_err(|e| PlaybackError::FormatError(format!("XML反序列化失败: {}", e)))?;

        // 重建时间戳索引（因为它被跳过序列化）
        index.build_timestamp_index();

        Ok(index)
    }

    /// 检查数据集目录是否有效
    pub fn validate_dataset_directory<P: AsRef<Path>>(dataset_path: P) -> Result<()> {
        let path = dataset_path.as_ref();

        if !path.exists() {
            return Err(PlaybackError::ProjectError(
                format!("数据集目录不存在: {:?}", path)
            ));
        }

        if !path.is_dir() {
            return Err(PlaybackError::ProjectError(
                format!("指定路径不是目录: {:?}", path)
            ));
        }

        Ok(())
    }

    /// 扫描目录中的PCAP文件
    pub fn scan_pcap_files<P: AsRef<Path>>(dir_path: P) -> Result<Vec<PathBuf>> {
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

    /// 检查索引是否需要重建
    pub fn needs_rebuild<P: AsRef<Path>>(
        index: &PidxIndex,
        dataset_path: P
    ) -> Result<bool> {
        let current_files = Self::scan_pcap_files(dataset_path)?;

        // 检查文件数量是否匹配
        if current_files.len() != index.files.len() {
            return Ok(true);
        }

        // 检查每个文件的哈希值
        for file_index in &index.files {
            if let Some(current_file) = current_files.iter()
                .find(|f| f.file_name().and_then(|n| n.to_str()) == Some(&file_index.file_name)) {

                match Self::calculate_file_hash(current_file) {
                    Ok(hash) => {
                        if hash != file_index.file_hash {
                            return Ok(true);
                        }
                    }
                    Err(_) => return Ok(true),
                }
            } else {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// 获取索引文件统计信息
    pub fn get_index_stats(index: &PidxIndex) -> IndexStats {
        IndexStats {
            total_files: index.files.len(),
            total_packets: index.total_packets,
            total_duration_ns: index.total_duration,
            start_timestamp: index.start_timestamp,
            end_timestamp: index.end_timestamp,
            average_packets_per_file: if index.files.is_empty() {
                0.0
            } else {
                index.total_packets as f64 / index.files.len() as f64
            },
        }
    }
}

/// 索引统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_files: usize,
    pub total_packets: u64,
    pub total_duration_ns: u64,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub average_packets_per_file: f64,
}

impl IndexStats {
    /// 获取总时长（秒）
    pub fn total_duration_seconds(&self) -> f64 {
        self.total_duration_ns as f64 / 1_000_000_000.0
    }

    /// 获取平均数据包速率（包/秒）
    pub fn average_packet_rate(&self) -> f64 {
        if self.total_duration_ns == 0 {
            0.0
        } else {
            self.total_packets as f64 / self.total_duration_seconds()
        }
    }
}

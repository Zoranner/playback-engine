use std::fs;
use std::path::{Path, PathBuf};
use log::{info, warn, debug};

use crate::types::{PlaybackError, Result, PidxIndex, PcapFileIndex, PacketIndexEntry};
use crate::pcap::reader::PcapReader;
use crate::pidx::reader::PidxReader;

/// PIDX文件写入器
pub struct PidxWriter;

impl PidxWriter {
    /// 保存索引到PIDX文件
    pub fn save_index<P: AsRef<Path>>(index: &PidxIndex, pidx_file_path: P) -> Result<()> {
        let xml_content = Self::serialize_to_xml(index)?;
        fs::write(pidx_file_path.as_ref(), xml_content)?;

        info!("PIDX索引文件已保存: {:?}", pidx_file_path.as_ref());
        Ok(())
    }

    /// 生成数据集的时间索引
    pub async fn generate_index<P: AsRef<Path>>(dataset_path: P) -> Result<PidxIndex> {
        let path = dataset_path.as_ref();
        let dataset_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("未命名数据集")
            .to_string();

        info!("开始生成数据集时间索引: {}", dataset_name);

        let mut index = PidxIndex::new(Some(format!("数据集: {}", dataset_name)));

        // 扫描目录中的所有PCAP文件
        let pcap_files = PidxReader::scan_pcap_files(path)?;

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

        // 更新统计信息
        index.update_time_range();
        index.update_total_packets();

        info!("索引生成完成 - 文件数: {}, 总数据包: {}, 时长: {:.2}秒",
              index.files.len(),
              index.total_packets,
              (index.end_timestamp - index.start_timestamp) as f64 / 1_000_000_000.0);

        Ok(index)
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
        let file_hash = PidxReader::calculate_file_hash(path)?;
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

    /// 将索引序列化为XML格式
    fn serialize_to_xml(index: &PidxIndex) -> Result<String> {
        let xml_string = serde_xml_rs::to_string(index)
            .map_err(|e| PlaybackError::FormatError(format!("XML序列化失败: {}", e)))?;

        // 添加XML声明
        let xml_with_declaration = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
{}"#, xml_string);

        Ok(xml_with_declaration)
    }

    /// 重建现有索引（增量更新）
    pub async fn rebuild_index<P: AsRef<Path>>(
        dataset_path: P,
        existing_index: Option<PidxIndex>
    ) -> Result<PidxIndex> {
        let path = dataset_path.as_ref();
        let current_files = PidxReader::scan_pcap_files(path)?;

        let mut index = existing_index.unwrap_or_else(|| {
            let dataset_name = path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("未命名数据集")
                .to_string();
            PidxIndex::new(Some(format!("数据集: {}", dataset_name)))
        });

        info!("重建索引，当前文件数: {}", current_files.len());

        // 清空现有文件索引
        index.files.clear();

        // 重新分析所有文件
        for file_path in current_files {
            match Self::index_pcap_file(&file_path).await {
                Ok(file_index) => {
                    index.files.push(file_index);
                }
                Err(e) => {
                    warn!("分析PCAP文件失败: {:?}, 错误: {}", file_path, e);
                }
            }
        }

        // 更新统计信息
        index.update_time_range();
        index.update_total_packets();

        info!("索引重建完成");
        Ok(index)
    }

    /// 保存索引的同时创建备份
    pub fn save_index_with_backup<P: AsRef<Path>>(
        index: &PidxIndex,
        pidx_file_path: P
    ) -> Result<()> {
        let path = pidx_file_path.as_ref();

        // 如果文件已存在，创建备份
        if path.exists() {
            let backup_path = path.with_extension("pidx.bak");
            fs::copy(path, &backup_path)?;
            info!("创建索引文件备份: {:?}", backup_path);
        }

        Self::save_index(index, path)
    }

    /// 生成索引并自动保存到数据集目录
    pub async fn generate_and_save_index<P: AsRef<Path>>(dataset_path: P) -> Result<PathBuf> {
        let path = dataset_path.as_ref();
        let index = Self::generate_index(path).await?;

        // 生成PIDX文件名
        let dataset_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("dataset");
        let pidx_filename = format!("{}.pidx", dataset_name);
        let pidx_file_path = path.join(pidx_filename);

        // 保存索引
        Self::save_index(&index, &pidx_file_path)?;

        Ok(pidx_file_path)
    }

    /// 验证并修复索引（如果需要）
    pub async fn validate_and_repair_index<P: AsRef<Path>>(
        index: PidxIndex,
        dataset_path: P
    ) -> Result<PidxIndex> {
        let path = dataset_path.as_ref();

        // 检查是否需要重建
        if PidxReader::needs_rebuild(&index, path)? {
            warn!("检测到索引需要重建");
            Self::rebuild_index(path, Some(index)).await
        } else {
            info!("索引验证通过，无需重建");
            Ok(index)
        }
    }
}

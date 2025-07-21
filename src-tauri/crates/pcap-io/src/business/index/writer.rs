use log::{debug, info, warn};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::foundation::error::{PcapError, Result};
use crate::data::file_reader::PcapFileReader;
use crate::business::index::reader::PidxReader;
use crate::business::index::types::{PacketIndexEntry, PcapFileIndex, PidxIndex};

/// PIDX文件写入器
///
/// 提供索引生成、保存等核心功能。
pub struct PidxWriter {
    /// 数据集目录路径
    dataset_path: PathBuf,
    /// 数据集名称
    dataset_name: String,
    /// 当前索引
    index: Option<PidxIndex>,
}

impl PidxWriter {
    /// 创建新的PIDX写入器
    ///
    /// # 参数
    /// - `dataset_path` - 数据集目录路径
    ///
    /// # 返回
    /// 返回初始化后的写入器实例
    pub fn new<P: AsRef<Path>>(dataset_path: P) -> Result<Self> {
        let path = dataset_path.as_ref().to_path_buf();

        if !path.exists() || !path.is_dir() {
            return Err(PcapError::InvalidArgument(format!(
                "数据集目录不存在或不是目录: {:?}",
                path
            )));
        }

        let dataset_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("未命名数据集")
            .to_string();

        Ok(Self {
            dataset_path: path,
            dataset_name,
            index: None,
        })
    }

    /// 生成并保存数据集的时间索引
    ///
    /// 生成当前数据集目录下所有PCAP文件的索引并保存到文件。
    pub fn generate_index(&mut self) -> Result<PathBuf> {
        info!("开始生成数据集时间索引: {}", self.dataset_name);

        let mut index = PidxIndex::new(Some(format!("数据集: {}", self.dataset_name)));

        // 扫描目录中的所有PCAP文件
        let pcap_files = PidxReader::scan_pcap_files(&self.dataset_path)?;

        if pcap_files.is_empty() {
            return Err(PcapError::InvalidFormat(
                "数据集目录中未找到PCAP文件".to_string(),
            ));
        }

        info!("找到 {} 个PCAP文件，开始分析...", pcap_files.len());

        let mut global_start_timestamp = u64::MAX;
        let mut global_end_timestamp = 0u64;
        let mut timestamp_index = HashMap::new();

        // 分析每个PCAP文件
        for file_path in pcap_files {
            match Self::index_pcap_file(&file_path) {
                Ok(file_index) => {
                    // 更新全局时间戳
                    if file_index.start_timestamp < global_start_timestamp {
                        global_start_timestamp = file_index.start_timestamp;
                    }
                    if file_index.end_timestamp > global_end_timestamp {
                        global_end_timestamp = file_index.end_timestamp;
                    }

                    // 构建时间戳索引
                    for packet in &file_index.data_packets {
                        timestamp_index.insert(packet.timestamp_ns, packet.clone());
                    }

                    index.data_files.files.push(file_index);
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

        // 设置时间戳索引
        index.timestamp_index = timestamp_index;

        // 内部保存索引
        self.index = Some(index);

        // 保存到文件
        let pidx_filename = format!("{}.pidx", self.dataset_name);
        let pidx_file_path = self.dataset_path.join(pidx_filename);

        let xml_content = Self::serialize_to_xml(&self.index.as_ref().unwrap())?;
        fs::write(&pidx_file_path, xml_content).map_err(|e| PcapError::Io(e))?;

        info!(
            "索引生成完成 - 文件数: {}, 总数据包: {}, 时长: {:.2}秒",
            self.index.as_ref().unwrap().data_files.files.len(),
            self.index.as_ref().unwrap().total_packets,
            (self.index.as_ref().unwrap().end_timestamp
                - self.index.as_ref().unwrap().start_timestamp) as f64
                / 1_000_000_000.0
        );

        info!("PIDX索引文件已保存: {:?}", pidx_file_path);
        Ok(pidx_file_path)
    }

    /// 为单个PCAP文件生成索引
    fn index_pcap_file<P: AsRef<Path>>(file_path: P) -> Result<PcapFileIndex> {
        let path = file_path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        debug!("正在分析PCAP文件: {}", file_name);

        // 计算文件哈希
        let file_hash = PidxReader::calculate_file_hash(path)?;
        let file_size = fs::metadata(path).map_err(|e| PcapError::Io(e))?.len();

        // 打开PCAP文件并读取所有数据包
        let mut reader = PcapFileReader::new(crate::business::config::Configuration::default());
        reader.open(path)?;
        let mut packets = Vec::new();
        let mut packet_count = 0u64;
        let mut current_position = 16u64; // PCAP文件头后的位置

        let mut start_timestamp = u64::MAX;
        let mut end_timestamp = 0u64;

        // 读取所有数据包并记录位置
        while let Some(packet) = reader.read_packet()? {
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
                byte_offset: current_position,
                packet_size: packet.packet_length() as u32,
            };

            packets.push(index_entry);
            packet_count += 1;

            // 更新当前位置（16字节包头 + 数据内容）
            current_position += 16 + packet.packet_length() as u64;
        }

        let file_index = PcapFileIndex {
            file_name,
            file_hash,
            file_size,
            packet_count,
            start_timestamp,
            end_timestamp,
            data_packets: packets,
        };

        debug!(
            "文件分析完成: {} 个数据包, 时间范围: {}ns - {}ns",
            packet_count, start_timestamp, end_timestamp
        );

        Ok(file_index)
    }

    /// 将索引序列化为XML格式
    fn serialize_to_xml(index: &PidxIndex) -> Result<String> {
        let xml_content = serde_xml_rs::to_string(index)
            .map_err(|e| PcapError::InvalidFormat(format!("XML序列化失败: {}", e)))?;
        Ok(xml_content)
    }
}

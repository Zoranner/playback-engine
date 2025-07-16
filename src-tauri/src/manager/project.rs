use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use log::{debug, info, warn};
use chrono::DateTime;

use crate::types::{ProjectInfo, PlaybackError, Result, PprojConfig, DatasetConfig};
use crate::pcap::multi_reader::MultiPcapReader;
use crate::pproj::{PprojReader, PprojWriter};
use crate::pidx::{PidxReader, PidxWriter};
use crate::pidx::reader::IndexStats;
use crate::types::PidxIndex;

/// 数据集读取器信息
pub struct DatasetReader {
    /// 数据集配置
    pub config: DatasetConfig,
    /// 多文件PCAP读取器
    pub reader: MultiPcapReader,
    /// PIDX索引
    pub index: PidxIndex,
}

/// 工程管理器
pub struct ProjectManager {
    /// 当前工程信息
    current_project: Option<ProjectInfo>,
    /// 工程配置
    project_config: Option<PprojConfig>,
    /// 数据集读取器映射
    dataset_readers: HashMap<String, DatasetReader>,
    /// 工程目录路径
    project_path: Option<PathBuf>,
}

impl ProjectManager {
    /// 创建新的工程管理器
    pub fn new() -> Self {
        Self {
            current_project: None,
            project_config: None,
            dataset_readers: HashMap::new(),
            project_path: None,
        }
    }

    /// 打开工程目录 - 支持完整的链式加载
    pub async fn open_project<P: AsRef<Path>>(&mut self, project_path: P) -> Result<ProjectInfo> {
        let path = project_path.as_ref();

        if !path.exists() {
            return Err(PlaybackError::ProjectError(
                format!("工程目录不存在: {:?}", path)
            ));
        }

        if !path.is_dir() {
            return Err(PlaybackError::ProjectError(
                format!("指定路径不是目录: {:?}", path)
            ));
        }

        info!("正在打开工程目录: {:?}", path);

        // 1. 查找并加载PPROJ文件
        let pproj_config = self.load_or_generate_pproj_config(path).await?;

        // 2. 初始化数据集读取器
        self.initialize_dataset_readers(&pproj_config).await?;

        // 3. 生成工程信息
        let project_info = self.generate_project_info(path, &pproj_config).await?;

        // 4. 保存状态
        self.project_path = Some(path.to_path_buf());
        self.project_config = Some(pproj_config);
        self.current_project = Some(project_info.clone());

        info!("工程打开成功: {}", project_info.name);
        Ok(project_info)
    }

    /// 加载或生成PPROJ配置
    async fn load_or_generate_pproj_config<P: AsRef<Path>>(&self, project_path: P) -> Result<PprojConfig> {
        let path = project_path.as_ref();

        // 查找现有的PPROJ文件
        if let Some(pproj_file) = PprojReader::find_pproj_file(path)? {
            info!("找到PPROJ文件: {:?}", pproj_file);
            PprojReader::load_config(pproj_file)
        } else {
            // 生成默认配置
            info!("未找到PPROJ文件，生成默认配置");
            let config = PprojWriter::generate_default_config(path)?;

            // 自动保存配置
            let project_name = PprojReader::suggest_project_name(path);
            let pproj_file_path = path.join(format!("{}.pproj", project_name));
            PprojWriter::save_config(&config, pproj_file_path)?;

            Ok(config)
        }
    }

    /// 初始化数据集读取器
    async fn initialize_dataset_readers(&mut self, config: &PprojConfig) -> Result<()> {
        self.dataset_readers.clear();

        for dataset_config in &config.datasets {
            match self.create_dataset_reader(dataset_config).await {
                Ok(reader) => {
                    self.dataset_readers.insert(dataset_config.name.clone(), reader);
                    info!("数据集读取器初始化成功: {}", dataset_config.name);
                }
                Err(e) => {
                    warn!("数据集读取器初始化失败: {}, 错误: {}", dataset_config.name, e);
                    // 继续处理其他数据集
                }
            }
        }

        info!("已初始化 {} 个数据集读取器", self.dataset_readers.len());
        Ok(())
    }

    /// 创建单个数据集读取器
    async fn create_dataset_reader(&self, dataset_config: &DatasetConfig) -> Result<DatasetReader> {
        let dataset_path = Path::new(&dataset_config.path);

        // 1. 查找或生成PIDX索引
        let index = self.load_or_generate_pidx_index(dataset_path).await?;

        // 2. 创建多文件读取器
        let reader = MultiPcapReader::new(dataset_path, index.clone())?;

        Ok(DatasetReader {
            config: dataset_config.clone(),
            reader,
            index,
        })
    }

    /// 加载或生成PIDX索引
    async fn load_or_generate_pidx_index<P: AsRef<Path>>(&self, dataset_path: P) -> Result<PidxIndex> {
        let path = dataset_path.as_ref();

        // 查找现有的PIDX文件
        if let Some(pidx_file) = PidxReader::find_pidx_file(path)? {
            info!("找到PIDX文件: {:?}", pidx_file);
            let index = PidxReader::load_index(pidx_file)?;

            // 验证索引有效性
            if PidxReader::verify_index_validity(&index, path).await? {
                return Ok(index);
            } else {
                warn!("PIDX文件验证失败，重新生成索引");
            }
        }

        // 生成新的索引
        info!("生成PIDX索引: {:?}", path);
        let index = PidxWriter::generate_index(path).await?;

        // 自动保存索引
        let dataset_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("dataset");
        let pidx_file_path = path.join(format!("{}.pidx", dataset_name));
        PidxWriter::save_index(&index, pidx_file_path)?;

        Ok(index)
    }

    /// 生成工程信息
    async fn generate_project_info<P: AsRef<Path>>(
        &self,
        project_path: P,
        config: &PprojConfig
    ) -> Result<ProjectInfo> {
        let path = project_path.as_ref();
        let project_name = config.project_name.clone();

        let mut project_info = ProjectInfo::new(
            project_name,
            path.to_string_lossy().to_string()
        );

        // 设置元数据
        project_info.metadata.description = config.project_description.clone();
        if let Some(author) = &config.author {
            project_info.metadata.participants.push(author.clone());
        }
        project_info.metadata.tags = config.tags.clone();

        // 统计所有数据集的信息
        let mut total_duration = 0u64;
        let mut total_files = 0usize;
        let mut earliest_time = u64::MAX;
        let mut latest_time = 0u64;
        let mut pcap_files = Vec::new();

        for dataset_reader in self.dataset_readers.values() {
            let stats = PidxReader::get_index_stats(&dataset_reader.index);

            total_duration += stats.total_duration_ns;
            total_files += stats.total_files;

            if stats.start_timestamp < earliest_time {
                earliest_time = stats.start_timestamp;
            }
            if stats.end_timestamp > latest_time {
                latest_time = stats.end_timestamp;
            }

            // 收集PCAP文件路径
            for file_index in &dataset_reader.index.files {
                let file_path = Path::new(&dataset_reader.config.path)
                    .join(&file_index.file_name);
                pcap_files.push(file_path.to_string_lossy().to_string());
            }
        }

        project_info.total_duration = total_duration;
        project_info.file_count = total_files;
        project_info.pcap_files = pcap_files;

        // 转换时间戳为ISO格式
        if earliest_time != u64::MAX {
            let start_time = DateTime::from_timestamp(
                (earliest_time / 1_000_000_000) as i64,
                (earliest_time % 1_000_000_000) as u32
            ).unwrap_or_default();
            project_info.start_time = start_time.to_rfc3339();
        }

        if latest_time != 0 {
            let end_time = DateTime::from_timestamp(
                (latest_time / 1_000_000_000) as i64,
                (latest_time % 1_000_000_000) as u32
            ).unwrap_or_default();
            project_info.end_time = end_time.to_rfc3339();
        }

        Ok(project_info)
    }

    /// 获取当前工程信息
    pub fn get_current_project(&self) -> Option<&ProjectInfo> {
        self.current_project.as_ref()
    }

    /// 获取工程配置
    pub fn get_project_config(&self) -> Option<&PprojConfig> {
        self.project_config.as_ref()
    }

    /// 获取数据集读取器
    pub fn get_dataset_reader(&self, dataset_name: &str) -> Option<&DatasetReader> {
        self.dataset_readers.get(dataset_name)
    }

    /// 获取数据集读取器（可变）
    pub fn get_dataset_reader_mut(&mut self, dataset_name: &str) -> Option<&mut DatasetReader> {
        self.dataset_readers.get_mut(dataset_name)
    }

    /// 列出所有数据集名称
    pub fn list_dataset_names(&self) -> Vec<String> {
        self.dataset_readers.keys().cloned().collect()
    }

    /// 刷新工程信息
    pub async fn refresh_project(&mut self) -> Result<ProjectInfo> {
        if let Some(project_path) = &self.project_path.clone() {
            self.open_project(project_path).await
        } else {
            Err(PlaybackError::ProjectError(
                "当前没有打开的工程".to_string()
            ))
        }
    }

    /// 关闭当前工程
    pub fn close_project(&mut self) {
        self.current_project = None;
        self.project_config = None;
        self.dataset_readers.clear();
        self.project_path = None;
        info!("工程已关闭");
    }

    /// 重新加载数据集索引
    pub async fn reload_dataset_index(&mut self, dataset_name: &str) -> Result<()> {
        if let Some(dataset_reader) = self.dataset_readers.get(dataset_name) {
            let dataset_path = &dataset_reader.config.path;
            let new_index = self.load_or_generate_pidx_index(dataset_path).await?;

            // 更新读取器
            let new_reader = MultiPcapReader::new(dataset_path, new_index.clone())?;

            self.dataset_readers.insert(dataset_name.to_string(), DatasetReader {
                config: dataset_reader.config.clone(),
                reader: new_reader,
                index: new_index,
            });

            info!("数据集索引已重新加载: {}", dataset_name);
            Ok(())
        } else {
            Err(PlaybackError::ProjectError(
                format!("数据集不存在: {}", dataset_name)
            ))
        }
    }

    /// 获取工程统计信息
    pub fn get_project_statistics(&self) -> ProjectStatistics {
        let mut stats = ProjectStatistics::default();

        if let Some(project) = &self.current_project {
            stats.total_duration_seconds = project.total_duration as f64 / 1_000_000_000.0;
            stats.total_files = project.file_count;
        }

        stats.total_datasets = self.dataset_readers.len();

        let mut total_packets = 0u64;
        for reader in self.dataset_readers.values() {
            total_packets += reader.index.total_packets;
        }
        stats.total_packets = total_packets;

        if stats.total_duration_seconds > 0.0 {
            stats.average_packet_rate = stats.total_packets as f64 / stats.total_duration_seconds;
        }

        stats
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 工程统计信息
#[derive(Debug, Clone, Default)]
pub struct ProjectStatistics {
    pub total_datasets: usize,
    pub total_files: usize,
    pub total_packets: u64,
    pub total_duration_seconds: f64,
    pub average_packet_rate: f64,
}

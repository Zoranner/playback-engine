use std::path::{Path, PathBuf};
use std::collections::HashMap;
use log::{info, warn};
// use chrono::DateTime;

use crate::types::{ProjectInfo, PlaybackError, Result, PprojConfig, DatasetConfig};
use crate::pproj::{PprojReader, PprojWriter};
use pcap_io::{Reader as PcapReader, Configuration, Read};

/// 数据集读取器信息
#[derive(Debug, Clone)]
pub struct DatasetReader {
    /// 数据集配置
    pub config: DatasetConfig,
    /// PCAP文件路径列表
    pub pcap_files: Vec<PathBuf>,
    /// 数据包总数（缓存）
    pub total_packets: u64,
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

    /// 打开工程目录
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

        // 扫描数据集目录中的所有PCAP文件
        let pcap_files = self.scan_pcap_files(dataset_path)?;

        if pcap_files.is_empty() {
            return Err(PlaybackError::ProjectError(format!(
                "数据集目录中未找到PCAP文件: {:?}", dataset_path
            )));
        }

        // 统计总数据包数
        let mut total_packets = 0;
        for file_path in &pcap_files {
            match self.count_packets_in_file(file_path) {
                Ok(count) => total_packets += count,
                Err(e) => warn!("统计文件 {:?} 数据包失败: {}", file_path, e),
            }
        }

        Ok(DatasetReader {
            config: dataset_config.clone(),
            pcap_files,
            total_packets,
        })
    }

    /// 扫描目录中的PCAP文件
    fn scan_pcap_files<P: AsRef<Path>>(&self, dir_path: P) -> Result<Vec<PathBuf>> {
        let mut pcap_files = Vec::new();
        let entries = std::fs::read_dir(dir_path)?;

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

    /// 统计单个文件中的数据包数量
    fn count_packets_in_file(&self, file_path: &Path) -> Result<u64> {
        let config = Configuration::default();
        let mut reader = PcapReader::new(file_path, config)
            .map_err(|e| PlaybackError::FileError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("打开PCAP文件失败: {}", e)
            )))?;

        let mut count = 0;
        while let Ok(Some(_)) = reader.read_packet() {
            count += 1;
        }

        Ok(count)
    }

    /// 生成工程信息
    async fn generate_project_info<P: AsRef<Path>>(
        &self,
        project_path: P,
        config: &PprojConfig,
    ) -> Result<ProjectInfo> {
        let path = project_path.as_ref();
        let project_name = config.project_name.clone();

        let mut project_info = ProjectInfo::new(project_name, path.to_string_lossy().to_string());

        // 设置元数据
        project_info.metadata.description = config.project_description.clone();
        if let Some(author) = &config.author {
            project_info.metadata.participants.push(author.clone());
        }
        project_info.metadata.tags = config.tags.clone();

        // 统计所有数据集的信息
        let mut total_files = 0usize;
        let mut pcap_files = Vec::new();

        for dataset_reader in self.dataset_readers.values() {
            total_files += dataset_reader.pcap_files.len();

            // 收集PCAP文件路径
            for file_path in &dataset_reader.pcap_files {
                pcap_files.push(file_path.to_string_lossy().to_string());
            }
        }

        project_info.file_count = total_files;
        project_info.pcap_files = pcap_files;

        Ok(project_info)
    }

    /// 获取当前工程信息
    pub fn get_current_project(&self) -> Option<&ProjectInfo> {
        self.current_project.as_ref()
    }

    /// 获取数据集读取器
    pub fn get_dataset_reader(&self, dataset_name: &str) -> Option<&DatasetReader> {
        self.dataset_readers.get(dataset_name)
    }

    /// 列出所有数据集名称
    pub fn list_dataset_names(&self) -> Vec<String> {
        self.dataset_readers.keys().cloned().collect()
    }

    /// 关闭当前工程
    pub fn close_project(&mut self) {
        self.current_project = None;
        self.project_config = None;
        self.dataset_readers.clear();
        self.project_path = None;
        info!("工程已关闭");
    }
}

/// 简化的统计信息结构
#[derive(Debug, Default)]
pub struct ProjectStatistics {
    pub total_datasets: usize,
    pub total_files: usize,
    pub total_packets: u64,
}

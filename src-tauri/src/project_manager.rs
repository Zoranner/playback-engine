use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use log::{debug, info, warn};
use chrono::DateTime;

use crate::types::{ProjectInfo, ProjectMetadata, PlaybackError, Result};
use crate::pcap_reader::PcapReader;
use crate::pproj::{PprojManager, PprojConfig, DatasetConfig};
use crate::pidx::{PidxManager, PidxIndex};
use crate::multi_pcap_reader::MultiPcapReader;

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

        // 第一步：查找或生成PPROJ工程文件
        let project_config = self.load_or_create_pproj_config(path).await?;
        info!("工程配置加载完成，包含 {} 个数据集", project_config.datasets.len());

        // 第二步：加载所有数据集
        self.load_all_datasets(&project_config).await?;

        // 第三步：分析工程信息
        let project_info = self.analyze_project_info(&project_config).await?;

        // 更新状态
        self.current_project = Some(project_info.clone());
        self.project_config = Some(project_config);
        self.project_path = Some(path.to_path_buf());

        info!("成功打开工程: {}", project_info.name);
        debug!("工程信息: {:?}", project_info);

        Ok(project_info)
    }

    /// 加载或创建PPROJ工程配置
    async fn load_or_create_pproj_config<P: AsRef<Path>>(&self, project_path: P) -> Result<PprojConfig> {
        let path = project_path.as_ref();

        // 查找现有的PPROJ文件
        if let Some(pproj_file) = PprojManager::find_pproj_file(path)? {
            info!("找到工程文件: {:?}", pproj_file);

            match PprojManager::load_config(&pproj_file) {
                Ok(config) => {
                    info!("成功加载现有工程配置");
                    return Ok(config);
                }
                Err(e) => {
                    warn!("加载工程文件失败: {}, 将生成新配置", e);
                }
            }
        }

        // 生成默认配置
        info!("生成默认工程配置");
        let config = PprojManager::generate_default_config(path)?;

        // 保存新生成的配置
        let pproj_file_name = format!("{}.pproj",
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("project")
        );
        let pproj_file_path = path.join(pproj_file_name);

        PprojManager::save_config(&config, &pproj_file_path)?;
        info!("新工程配置已保存: {:?}", pproj_file_path);

        Ok(config)
    }

    /// 加载所有数据集
    async fn load_all_datasets(&mut self, project_config: &PprojConfig) -> Result<()> {
        info!("开始加载 {} 个数据集", project_config.datasets.len());

        self.dataset_readers.clear();

        for dataset_config in &project_config.datasets {
            if !dataset_config.enabled {
                info!("跳过禁用的数据集: {}", dataset_config.name);
                continue;
            }

            match self.load_single_dataset(dataset_config.clone()).await {
                Ok(dataset_reader) => {
                    info!("成功加载数据集: {} ({} 个数据包)",
                          dataset_config.name,
                          dataset_reader.reader.get_total_packets());

                    self.dataset_readers.insert(dataset_config.name.clone(), dataset_reader);
                }
                Err(e) => {
                    warn!("加载数据集失败: {}, 错误: {}", dataset_config.name, e);

                    // 根据配置决定是否继续加载其他数据集
                    // 这里选择继续加载，但记录错误
                    continue;
                }
            }
        }

        if self.dataset_readers.is_empty() {
            return Err(PlaybackError::ProjectError(
                "没有成功加载任何数据集".to_string()
            ));
        }

        info!("数据集加载完成，成功加载 {} 个数据集", self.dataset_readers.len());
        Ok(())
    }

    /// 加载单个数据集
    async fn load_single_dataset(&self, dataset_config: DatasetConfig) -> Result<DatasetReader> {
        let dataset_path = Path::new(&dataset_config.path);

        debug!("加载数据集: {} -> {:?}", dataset_config.name, dataset_path);

        // 验证数据集路径
        if !dataset_path.exists() {
            return Err(PlaybackError::ProjectError(
                format!("数据集路径不存在: {:?}", dataset_path)
            ));
        }

        // 创建多文件PCAP读取器（自动处理PIDX索引）
        let multi_reader = MultiPcapReader::from_dataset(dataset_path).await?;
        let index = multi_reader.get_index().clone();

        Ok(DatasetReader {
            config: dataset_config,
            reader: multi_reader,
            index,
        })
    }

    /// 分析工程信息
    async fn analyze_project_info(&self, project_config: &PprojConfig) -> Result<ProjectInfo> {
        let mut project_info = ProjectInfo::new(
            project_config.project_name.clone(),
            self.project_path.as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default()
        );

        // 设置基本信息
        if let Some(desc) = &project_config.project_description {
            project_info.metadata.description = Some(desc.clone());
        }

        if let Some(author) = &project_config.author {
            project_info.metadata.participants.push(author.clone());
        }

        project_info.metadata.tags = project_config.tags.clone();

        // 统计所有数据集的信息
        let mut total_duration = 0u64;
        let mut total_packets = 0u64;
        let mut earliest_time = u64::MAX;
        let mut latest_time = 0u64;
        let mut all_pcap_files = Vec::new();

        for (dataset_name, dataset_reader) in &self.dataset_readers {
            let duration = dataset_reader.reader.get_total_duration();
            let packets = dataset_reader.reader.get_total_packets();
            let start_time = dataset_reader.reader.get_start_timestamp();
            let end_time = dataset_reader.reader.get_end_timestamp();

                    total_duration += duration;
            total_packets += packets;

            if start_time < earliest_time {
                earliest_time = start_time;
            }
            if end_time > latest_time {
                latest_time = end_time;
            }

            // 收集所有PCAP文件路径
            for file_name in dataset_reader.reader.get_file_list() {
                let full_path = dataset_reader.reader.get_dataset_path().join(file_name);
                all_pcap_files.push(full_path.to_string_lossy().to_string());
            }

            debug!("数据集 {}: {} 数据包, {:.2}秒",
                   dataset_name, packets, duration as f64 / 1_000_000_000.0);
        }

        // 更新工程信息
        project_info.total_duration = total_duration;
        project_info.file_count = all_pcap_files.len();
        project_info.pcap_files = all_pcap_files;

        // 转换时间戳为ISO格式字符串
        if earliest_time != u64::MAX {
            project_info.start_time = self.timestamp_to_iso_string(earliest_time);
        }
        if latest_time != 0 {
            project_info.end_time = self.timestamp_to_iso_string(latest_time);
        }

        info!("工程分析完成 - 数据集数: {}, 文件数: {}, 总数据包: {}, 总时长: {:.2}秒",
              self.dataset_readers.len(),
              project_info.file_count,
              total_packets,
              total_duration as f64 / 1_000_000_000.0);

        Ok(project_info)
    }

    /// 将纳秒时间戳转换为ISO格式字符串
    fn timestamp_to_iso_string(&self, timestamp_ns: u64) -> String {
        let timestamp_secs = (timestamp_ns / 1_000_000_000) as i64;
        let nanosecs = (timestamp_ns % 1_000_000_000) as u32;

        if let Some(datetime) = DateTime::from_timestamp(timestamp_secs, nanosecs) {
            datetime.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
        } else {
            "1970-01-01T00:00:00.000Z".to_string()
        }
    }

    /// 获取指定数据集的读取器
    pub fn get_dataset_reader(&self, dataset_name: &str) -> Option<&DatasetReader> {
        self.dataset_readers.get(dataset_name)
    }

    /// 获取指定数据集的读取器（可变）
    pub fn get_dataset_reader_mut(&mut self, dataset_name: &str) -> Option<&mut DatasetReader> {
        self.dataset_readers.get_mut(dataset_name)
    }

    /// 获取所有数据集名称
    pub fn get_dataset_names(&self) -> Vec<&str> {
        self.dataset_readers.keys().map(|s| s.as_str()).collect()
    }

    /// 获取工程配置
    pub fn get_project_config(&self) -> Option<&PprojConfig> {
        self.project_config.as_ref()
    }

    /// 保存工程配置
    pub fn save_project_config(&self) -> Result<()> {
        if let (Some(config), Some(project_path)) = (&self.project_config, &self.project_path) {
            let pproj_file_name = format!("{}.pproj",
                project_path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("project")
            );
            let pproj_file_path = project_path.join(pproj_file_name);

            PprojManager::save_config(config, &pproj_file_path)?;
            info!("工程配置已保存: {:?}", pproj_file_path);
        }

        Ok(())
    }

    /// 重新生成指定数据集的PIDX索引
    pub async fn regenerate_dataset_index(&mut self, dataset_name: &str) -> Result<()> {
        if let Some(dataset_reader) = self.dataset_readers.get(dataset_name) {
            let dataset_path = &dataset_reader.config.path;

            info!("重新生成数据集索引: {}", dataset_name);

            // 生成新索引
            let new_index = PidxManager::generate_index(dataset_path).await?;

            // 保存索引
            let pidx_file_path = MultiPcapReader::get_pidx_file_path(dataset_path);
            PidxManager::save_index(&new_index, &pidx_file_path)?;

            // 重新加载数据集
            let dataset_config = dataset_reader.config.clone();
            let new_dataset_reader = self.load_single_dataset(dataset_config).await?;

            self.dataset_readers.insert(dataset_name.to_string(), new_dataset_reader);

            info!("数据集索引重新生成完成: {}", dataset_name);
        } else {
            return Err(PlaybackError::ProjectError(
                format!("数据集不存在: {}", dataset_name)
            ));
        }

        Ok(())
    }

    /// 根据时间戳从指定数据集读取数据包
    pub fn read_packet_from_dataset(&mut self, dataset_name: &str, timestamp: u64) -> Result<Option<crate::types::DataPacket>> {
        if let Some(dataset_reader) = self.dataset_readers.get_mut(dataset_name) {
            dataset_reader.reader.read_packet_at_time(timestamp)
        } else {
            Err(PlaybackError::ProjectError(
                format!("数据集不存在: {}", dataset_name)
            ))
        }
    }

    /// 从指定数据集读取时间范围内的数据包
    pub fn read_packets_from_dataset_range(
        &mut self,
        dataset_name: &str,
        start_time: u64,
        end_time: u64
    ) -> Result<Vec<crate::types::DataPacket>> {
        if let Some(dataset_reader) = self.dataset_readers.get_mut(dataset_name) {
            dataset_reader.reader.read_packets_in_range(start_time, end_time)
        } else {
            Err(PlaybackError::ProjectError(
                format!("数据集不存在: {}", dataset_name)
            ))
        }
    }

    /// 从所有数据集读取指定时间范围的数据包
    pub fn read_packets_from_all_datasets(&mut self, start_time: u64, end_time: u64) -> Result<Vec<(String, Vec<crate::types::DataPacket>)>> {
        let mut all_packets = Vec::new();

        for (dataset_name, dataset_reader) in &mut self.dataset_readers {
            match dataset_reader.reader.read_packets_in_range(start_time, end_time) {
                Ok(packets) => {
                    all_packets.push((dataset_name.clone(), packets));
                }
                Err(e) => {
                    warn!("从数据集 {} 读取数据包失败: {}", dataset_name, e);
                }
            }
        }

        Ok(all_packets)
    }

    /// 获取当前工程信息
    pub fn get_current_project(&self) -> Option<&ProjectInfo> {
        self.current_project.as_ref()
    }

    /// 关闭当前工程
    pub fn close_project(&mut self) {
        if let Some(project) = &self.current_project {
            info!("关闭工程: {}", project.name);
        }

        self.current_project = None;
        self.project_config = None;
        self.dataset_readers.clear();
        self.project_path = None;
    }

    /// 验证工程目录
    pub fn validate_project_directory<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(PlaybackError::ProjectError(
                format!("目录不存在: {:?}", path)
            ));
        }

        if !path.is_dir() {
            return Err(PlaybackError::ProjectError(
                format!("路径不是目录: {:?}", path)
            ));
        }

        // 检查是否有数据集子目录或PCAP文件
        let entries = fs::read_dir(path)?;
        let mut has_datasets = false;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // 检查子目录是否包含PCAP文件
                if Self::directory_has_pcap_files(&entry_path)? {
                    has_datasets = true;
                    break;
                }
            } else if entry_path.extension().and_then(|ext| ext.to_str()) == Some("pcap") {
                has_datasets = true;
                break;
            }
        }

        if !has_datasets {
            return Err(PlaybackError::ProjectError(
                "目录中未找到PCAP文件或包含PCAP文件的数据集子目录".to_string()
            ));
        }

        Ok(())
    }

    /// 检查目录是否包含PCAP文件
    fn directory_has_pcap_files<P: AsRef<Path>>(dir_path: P) -> Result<bool> {
        let entries = fs::read_dir(dir_path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.to_str() == Some("pcap") {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    // 兼容性方法：保持与现有代码的兼容性

    /// 扫描目录中的PCAP文件（兼容性方法）
    #[deprecated(note = "使用新的数据集管理方式")]
    fn scan_pcap_files<P: AsRef<Path>>(&self, dir_path: P) -> Result<Vec<PathBuf>> {
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

        pcap_files.sort();
        Ok(pcap_files)
    }

    /// 获取PCAP文件列表（兼容性方法）
    #[deprecated(note = "使用 get_dataset_reader 和相关方法")]
    pub fn get_pcap_files(&self) -> Vec<PathBuf> {
        let mut all_files = Vec::new();

        for dataset_reader in self.dataset_readers.values() {
            for file_name in dataset_reader.reader.get_file_list() {
                let file_path = dataset_reader.reader.get_dataset_path().join(file_name);
                all_files.push(file_path);
            }
        }

        all_files
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

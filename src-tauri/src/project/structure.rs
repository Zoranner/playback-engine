use log::{debug, info, warn};
use std::fs;
use std::path::{Path, PathBuf};

use crate::types::common::{ProjectInfo, Result};

/// 工程结构表示
pub struct ProjectStructure {
    pub root_path: PathBuf,
    pub name: String,
    pub datasets: Vec<DatasetStructure>,
}

/// 数据集结构
pub struct DatasetStructure {
    pub name: String,
    pub path: PathBuf,
    pub pcap_files: Vec<PathBuf>,
    pub index_files: Vec<PathBuf>,
}

impl ProjectStructure {
    /// 从路径创建工程结构
    pub fn from_path<P: AsRef<Path>>(project_path: P) -> Result<Self> {
        let root_path = project_path.as_ref().to_path_buf();
        let name = root_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("untitled")
            .to_string();

        let mut datasets = Vec::new();

        // 扫描数据集目录
        info!("开始扫描工程目录: {:?}", root_path);
        if let Ok(entries) = std::fs::read_dir(&root_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        debug!("发现目录: {:?}", path);
                        match Self::scan_dataset(&path) {
                            Ok(dataset) => {
                                info!(
                                    "扫描数据集 '{}': {} 个PCAP文件, {} 个索引文件",
                                    dataset.name,
                                    dataset.pcap_files.len(),
                                    dataset.index_files.len()
                                );
                                // 显示所有数据集目录，不论是否包含文件
                                datasets.push(dataset);
                            }
                            Err(e) => {
                                warn!("扫描数据集目录失败 {:?}: {}", path, e);
                            }
                        }
                    } else {
                        debug!("跳过非目录项: {:?}", path);
                    }
                }
            }
        } else {
            warn!("无法读取工程目录: {:?}", root_path);
        }

        // 按数据集名称排序
        datasets.sort_by(|a, b| a.name.cmp(&b.name));

        info!("工程扫描完成，共发现 {} 个数据集", datasets.len());
        for dataset in &datasets {
            info!(
                "  - {}: {} 个文件",
                dataset.name,
                dataset.pcap_files.len() + dataset.index_files.len()
            );
        }

        Ok(ProjectStructure {
            root_path,
            name,
            datasets,
        })
    }

    /// 扫描单个数据集
    fn scan_dataset<P: AsRef<Path>>(dataset_path: P) -> Result<DatasetStructure> {
        let path = dataset_path.as_ref().to_path_buf();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut pcap_files = Vec::new();
        let mut index_files = Vec::new();

        // 扫描PCAP文件和索引文件
        debug!("开始扫描数据集目录: {:?}", path);
        if let Ok(entries) = std::fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        if let Some(ext) = file_path.extension() {
                            match ext.to_str() {
                                Some("pcap") => {
                                    debug!("发现PCAP文件: {:?}", file_path);
                                    pcap_files.push(file_path);
                                }
                                Some("pidx") => {
                                    debug!("发现索引文件: {:?}", file_path);
                                    index_files.push(file_path);
                                }
                                _ => {
                                    debug!("跳过其他文件: {:?}", file_path);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            warn!("无法读取数据集目录: {:?}", path);
        }

        // 按文件名排序
        pcap_files.sort();
        index_files.sort();

        Ok(DatasetStructure {
            name,
            path,
            pcap_files,
            index_files,
        })
    }

    /// 转换为项目信息
    pub fn to_project_info(&self) -> Result<ProjectInfo> {
        let mut project_info = ProjectInfo::new(
            self.name.clone(),
            self.root_path.to_string_lossy().to_string(),
        );

        // 设置文件计数
        project_info.file_count = self
            .datasets
            .iter()
            .map(|d| d.pcap_files.len())
            .sum::<usize>();

        // 收集所有PCAP文件路径
        project_info.pcap_files = self
            .datasets
            .iter()
            .flat_map(|d| d.pcap_files.iter())
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        // 尝试获取时间范围（从第一个和最后一个文件的元数据）
        if let Some(first_file) = project_info.pcap_files.first() {
            if let Ok(metadata) = fs::metadata(first_file) {
                if let Ok(created) = metadata.created() {
                    project_info.start_time = created
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                        .to_string();
                }
            }
        }

        if let Some(last_file) = project_info.pcap_files.last() {
            if let Ok(metadata) = fs::metadata(last_file) {
                if let Ok(modified) = metadata.modified() {
                    project_info.end_time = modified
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                        .to_string();
                }
            }
        }

        Ok(project_info)
    }
}

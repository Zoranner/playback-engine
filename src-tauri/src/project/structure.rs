use std::path::{Path, PathBuf};

use crate::types::common::{Result, ProjectInfo};

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
}

impl ProjectStructure {
    /// 从路径创建工程结构
    pub fn from_path<P: AsRef<Path>>(project_path: P) -> Result<Self> {
        let root_path = project_path.as_ref().to_path_buf();
        let name = root_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("untitled")
            .to_string();

        let mut datasets = Vec::new();

        // 扫描数据集目录
        if let Ok(entries) = std::fs::read_dir(&root_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let dataset = Self::scan_dataset(&path)?;
                        datasets.push(dataset);
                    }
                }
            }
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
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut pcap_files = Vec::new();

        // 扫描PCAP文件
        if let Ok(entries) = std::fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        if let Some(ext) = file_path.extension() {
                            if ext == "pcap" {
                                pcap_files.push(file_path);
                            }
                        }
                    }
                }
            }
        }

        // 按文件名排序
        pcap_files.sort();

        Ok(DatasetStructure {
            name,
            path,
            pcap_files,
        })
    }

    /// 转换为项目信息
    pub fn to_project_info(&self) -> Result<ProjectInfo> {
        let mut project_info = ProjectInfo::new(
            self.name.clone(),
            self.root_path.to_string_lossy().to_string(),
        );

        // 设置文件计数
        project_info.file_count = self.datasets.iter()
            .map(|d| d.pcap_files.len())
            .sum::<usize>();

        Ok(project_info)
    }
}
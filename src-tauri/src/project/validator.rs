//! 工程验证器

use crate::types::{PlaybackError, Result};

/// 验证工程结构
pub struct ProjectValidator;

impl ProjectValidator {
    /// 验证工程路径是否有效
    pub fn validate_project_path(project_path: &str) -> Result<()> {
        let path = std::path::Path::new(project_path);

        if !path.exists() {
            return Err(PlaybackError::ProjectError(format!(
                "工程路径不存在: {}",
                project_path
            )));
        }

        if !path.is_dir() {
            return Err(PlaybackError::ProjectError(format!(
                "工程路径不是目录: {}",
                project_path
            )));
        }

        Ok(())
    }

    /// 验证数据集是否包含PCAP文件
    pub fn validate_dataset(dataset_path: &str) -> Result<usize> {
        let path = std::path::Path::new(dataset_path);

        if !path.exists() || !path.is_dir() {
            return Err(PlaybackError::ProjectError(format!(
                "数据集路径无效: {}",
                dataset_path
            )));
        }

        let mut pcap_count = 0;
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        if let Some(ext) = file_path.extension() {
                            if ext == "pcap" {
                                pcap_count += 1;
                            }
                        }
                    }
                }
            }
        }

        if pcap_count == 0 {
            return Err(PlaybackError::ProjectError(format!(
                "数据集不包含PCAP文件: {}",
                dataset_path
            )));
        }

        Ok(pcap_count)
    }
}

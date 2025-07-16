use std::fs;
use std::path::{Path, PathBuf};
use log::{info, warn};

use crate::types::{PlaybackError, Result, PprojConfig};

/// PPROJ文件读取器
pub struct PprojReader;

impl PprojReader {
    /// 从PPROJ文件加载工程配置
    pub fn load_config<P: AsRef<Path>>(pproj_file_path: P) -> Result<PprojConfig> {
        let xml_content = fs::read_to_string(pproj_file_path.as_ref())?;
        let config = Self::deserialize_from_xml(&xml_content)?;

        // 验证配置
        config.validate()?;

        info!("PPROJ工程文件已加载: {:?}", pproj_file_path.as_ref());
        Ok(config)
    }

    /// 查找工程目录中的PPROJ文件
    pub fn find_pproj_file<P: AsRef<Path>>(project_dir: P) -> Result<Option<PathBuf>> {
        let entries = fs::read_dir(project_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.to_str() == Some("pproj") {
                        return Ok(Some(path));
                    }
                }
            }
        }

        Ok(None)
    }

    /// 从XML格式反序列化工程配置
    fn deserialize_from_xml(xml_content: &str) -> Result<PprojConfig> {
        let config: PprojConfig = serde_xml_rs::from_str(xml_content)
            .map_err(|e| PlaybackError::FormatError(format!("XML反序列化失败: {}", e)))?;

        Ok(config)
    }

    /// 验证工程目录是否有效
    pub fn validate_project_directory<P: AsRef<Path>>(project_dir: P) -> Result<()> {
        let path = project_dir.as_ref();

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

        Ok(())
    }

    /// 检查目录中是否包含PCAP文件
    pub fn has_pcap_files<P: AsRef<Path>>(dir_path: P) -> Result<bool> {
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

    /// 扫描工程目录，查找所有数据集子目录
    pub fn scan_datasets<P: AsRef<Path>>(project_dir: P) -> Result<Vec<PathBuf>> {
        let path = project_dir.as_ref();
        let mut datasets = Vec::new();

        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // 检查目录中是否有PCAP文件
                if Self::has_pcap_files(&entry_path)? {
                    datasets.push(entry_path);
                }
            }
        }

        datasets.sort();
        Ok(datasets)
    }

    /// 从目录路径获取建议的工程名称
    pub fn suggest_project_name<P: AsRef<Path>>(project_dir: P) -> String {
        project_dir.as_ref()
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("未命名工程")
            .to_string()
    }
}

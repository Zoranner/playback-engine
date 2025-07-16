use std::fs;
use std::path::Path;
use log::info;

use crate::types::{PlaybackError, Result, PprojConfig, DatasetConfig, NetworkConfig};

/// PPROJ文件写入器
pub struct PprojWriter;

impl PprojWriter {
    /// 保存工程配置到PPROJ文件
    pub fn save_config<P: AsRef<Path>>(config: &PprojConfig, pproj_file_path: P) -> Result<()> {
        let xml_content = Self::serialize_to_xml(config)?;
        fs::write(pproj_file_path.as_ref(), xml_content)?;

        info!("PPROJ工程文件已保存: {:?}", pproj_file_path.as_ref());
        Ok(())
    }

    /// 从目录生成默认工程配置
    pub fn generate_default_config<P: AsRef<Path>>(project_dir: P) -> Result<PprojConfig> {
        let path = project_dir.as_ref();
        let project_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("未命名工程")
            .to_string();

        info!("为目录 {:?} 生成默认工程配置", path);

        let mut config = PprojConfig::new(project_name)
            .with_description("自动生成的工程配置".to_string());

        // 扫描子目录作为数据集
        let entries = fs::read_dir(path)?;
        let mut port_counter = 5000u16;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // 检查目录中是否有PCAP文件
                if Self::has_pcap_files(&entry_path)? {
                    let dataset_name = entry_path.file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("unnamed")
                        .to_string();

                    // 为每个数据集分配不同的端口
                    let network_config = NetworkConfig::multicast(
                        "239.255.255.250",
                        port_counter
                    );
                    port_counter += 1;

                    let dataset_config = DatasetConfig::new(
                        dataset_name.clone(),
                        entry_path.clone()
                    )
                    .with_network_config(network_config)
                    .with_description(format!("数据集: {}", dataset_name));

                    config = config.add_dataset(dataset_config);

                    info!("添加数据集: {} -> {:?}", dataset_name, entry_path);
                }
            }
        }

        if config.datasets.is_empty() {
            return Err(PlaybackError::ProjectError(
                "工程目录中未找到包含PCAP文件的数据集".to_string()
            ));
        }

        config.validate()?;
        Ok(config)
    }

    /// 将工程配置序列化为XML格式
    fn serialize_to_xml(config: &PprojConfig) -> Result<String> {
        let xml_string = serde_xml_rs::to_string(config)
            .map_err(|e| PlaybackError::FormatError(format!("XML序列化失败: {}", e)))?;

        // 添加XML声明
        let xml_with_declaration = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
{}"#, xml_string);

        Ok(xml_with_declaration)
    }

    /// 检查目录中是否包含PCAP文件
    fn has_pcap_files<P: AsRef<Path>>(dir_path: P) -> Result<bool> {
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

    /// 创建基础工程配置模板
    pub fn create_template_config(project_name: String) -> PprojConfig {
        PprojConfig::new(project_name)
            .with_description("新建工程".to_string())
            .with_author("System".to_string())
            .add_tag("template".to_string())
    }

    /// 批量添加数据集到配置
    pub fn add_datasets_to_config(
        mut config: PprojConfig,
        dataset_paths: Vec<std::path::PathBuf>
    ) -> Result<PprojConfig> {
        let mut port_counter = 5000u16;

        for dataset_path in dataset_paths {
            let dataset_name = dataset_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unnamed")
                .to_string();

            let network_config = NetworkConfig::multicast(
                "239.255.255.250",
                port_counter
            );
            port_counter += 1;

            let dataset_config = DatasetConfig::new(
                dataset_name.clone(),
                dataset_path.clone()
            )
            .with_network_config(network_config);

            config = config.add_dataset(dataset_config);
        }

        config.validate()?;
        Ok(config)
    }

    /// 保存配置的同时创建备份
    pub fn save_config_with_backup<P: AsRef<Path>>(
        config: &PprojConfig,
        pproj_file_path: P
    ) -> Result<()> {
        let path = pproj_file_path.as_ref();

        // 如果文件已存在，创建备份
        if path.exists() {
            let backup_path = path.with_extension("pproj.bak");
            fs::copy(path, &backup_path)?;
            info!("创建配置文件备份: {:?}", backup_path);
        }

        Self::save_config(config, path)
    }
}

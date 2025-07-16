use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use log::{info, warn};

use crate::types::{PlaybackError, Result};

/// 网络传输类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NetworkType {
    Unicast,   // 单播
    Multicast, // 组播
    Broadcast, // 广播
}

impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::Multicast
    }
}

impl std::fmt::Display for NetworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkType::Unicast => write!(f, "unicast"),
            NetworkType::Multicast => write!(f, "multicast"),
            NetworkType::Broadcast => write!(f, "broadcast"),
        }
    }
}

impl std::str::FromStr for NetworkType {
    type Err = PlaybackError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "unicast" => Ok(NetworkType::Unicast),
            "multicast" => Ok(NetworkType::Multicast),
            "broadcast" => Ok(NetworkType::Broadcast),
            _ => Err(PlaybackError::ParseError(
                format!("未知的网络类型: {}", s)
            )),
        }
    }
}

/// 网络配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "network_config")]
pub struct NetworkConfig {
    /// 网络类型
    pub network_type: NetworkType,
    /// IP地址
    pub ip_address: String,
    /// 端口号
    pub port: u16,
    /// 网络接口（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    /// 是否启用
    pub enabled: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            network_type: NetworkType::Multicast,
            ip_address: "239.255.255.250".to_string(), // 默认组播地址
            port: 5000,
            interface: None,
            enabled: true,
        }
    }
}

impl NetworkConfig {
    /// 创建单播配置
    pub fn unicast(ip: &str, port: u16) -> Self {
        Self {
            network_type: NetworkType::Unicast,
            ip_address: ip.to_string(),
            port,
            interface: None,
            enabled: true,
        }
    }

    /// 创建组播配置
    pub fn multicast(ip: &str, port: u16) -> Self {
        Self {
            network_type: NetworkType::Multicast,
            ip_address: ip.to_string(),
            port,
            interface: None,
            enabled: true,
        }
    }

    /// 创建广播配置
    pub fn broadcast(port: u16) -> Self {
        Self {
            network_type: NetworkType::Broadcast,
            ip_address: "255.255.255.255".to_string(),
            port,
            interface: None,
            enabled: true,
        }
    }

    /// 验证网络配置
    pub fn validate(&self) -> Result<()> {
        // 验证IP地址格式
        if let Err(_) = self.ip_address.parse::<std::net::IpAddr>() {
            return Err(PlaybackError::ParseError(
                format!("无效的IP地址: {}", self.ip_address)
            ));
        }

        // 验证端口范围
        if self.port == 0 {
            return Err(PlaybackError::ParseError(
                "端口号不能为0".to_string()
            ));
        }

        // 验证组播地址范围
        if self.network_type == NetworkType::Multicast {
            if let Ok(ip) = self.ip_address.parse::<std::net::Ipv4Addr>() {
                if !ip.is_multicast() {
                    return Err(PlaybackError::ParseError(
                        format!("非组播地址: {}", self.ip_address)
                    ));
                }
            }
        }

        Ok(())
    }
}

/// 数据集配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "dataset")]
pub struct DatasetConfig {
    /// 数据集名称
    pub name: String,
    /// 数据集目录路径
    pub path: String,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否启用
    pub enabled: bool,
    /// 网络配置
    pub network_config: NetworkConfig,
    /// PIDX索引文件路径（如果存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pidx_file: Option<String>,
    /// 数据集标签
    #[serde(rename = "tag")]
    pub tags: Vec<String>,
}

impl DatasetConfig {
    /// 创建新的数据集配置
    pub fn new<P: AsRef<Path>>(name: String, path: P) -> Self {
        let path_str = path.as_ref().to_string_lossy().to_string();

        Self {
            name: name.clone(),
            path: path_str.clone(),
            description: None,
            enabled: true,
            network_config: NetworkConfig::default(),
            pidx_file: None,
            tags: Vec::new(),
        }
    }

    /// 设置网络配置
    pub fn with_network_config(mut self, config: NetworkConfig) -> Self {
        self.network_config = config;
        self
    }

    /// 设置描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 添加标签
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// 设置PIDX文件路径
    pub fn with_pidx_file(mut self, pidx_file: String) -> Self {
        self.pidx_file = Some(pidx_file);
        self
    }

    /// 验证数据集配置
    pub fn validate(&self) -> Result<()> {
        // 验证路径是否存在
        let path = Path::new(&self.path);
        if !path.exists() {
            return Err(PlaybackError::ProjectError(
                format!("数据集路径不存在: {}", self.path)
            ));
        }

        if !path.is_dir() {
            return Err(PlaybackError::ProjectError(
                format!("数据集路径不是目录: {}", self.path)
            ));
        }

        // 验证网络配置
        self.network_config.validate()?;

        // 验证PIDX文件（如果指定）
        if let Some(pidx_file) = &self.pidx_file {
            let pidx_path = Path::new(pidx_file);
            if !pidx_path.exists() {
                warn!("指定的PIDX文件不存在: {}", pidx_file);
            }
        }

        Ok(())
    }
}

/// PPROJ工程文件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "pproj_config")]
pub struct PprojConfig {
    /// 工程创建时间
    pub created_at: String,
    /// 工程版本
    pub version: String,
    /// 工程名称
    pub project_name: String,
    /// 工程描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_description: Option<String>,
    /// 工程作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// 数据集配置列表
    #[serde(rename = "dataset")]
    pub datasets: Vec<DatasetConfig>,
    /// 全局网络设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_settings: Option<NetworkConfig>,
    /// 工程标签
    #[serde(rename = "tag")]
    pub tags: Vec<String>,
}

impl PprojConfig {
    /// 创建新的工程配置
    pub fn new(project_name: String) -> Self {
        Self {
            created_at: chrono::Utc::now().to_rfc3339(),
            version: "1.0".to_string(),
            project_name,
            project_description: None,
            author: None,
            datasets: Vec::new(),
            global_network_settings: None,
            tags: Vec::new(),
        }
    }

    /// 添加数据集
    pub fn add_dataset(mut self, dataset: DatasetConfig) -> Self {
        self.datasets.push(dataset);
        self
    }

    /// 设置描述
    pub fn with_description(mut self, description: String) -> Self {
        self.project_description = Some(description);
        self
    }

    /// 设置作者
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// 设置全局网络设置
    pub fn with_global_network_settings(mut self, settings: NetworkConfig) -> Self {
        self.global_network_settings = Some(settings);
        self
    }

    /// 添加标签
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// 查找数据集
    pub fn find_dataset(&self, name: &str) -> Option<&DatasetConfig> {
        self.datasets.iter().find(|ds| ds.name == name)
    }

    /// 查找数据集（可变）
    pub fn find_dataset_mut(&mut self, name: &str) -> Option<&mut DatasetConfig> {
        self.datasets.iter_mut().find(|ds| ds.name == name)
    }

    /// 删除数据集
    pub fn remove_dataset(&mut self, name: &str) -> bool {
        if let Some(pos) = self.datasets.iter().position(|ds| ds.name == name) {
            self.datasets.remove(pos);
            true
        } else {
            false
        }
    }

    /// 验证工程配置
    pub fn validate(&self) -> Result<()> {
        // 验证数据集名称唯一性
        let mut names = std::collections::HashSet::new();
        for dataset in &self.datasets {
            if !names.insert(&dataset.name) {
                return Err(PlaybackError::ProjectError(
                    format!("重复的数据集名称: {}", dataset.name)
                ));
            }
        }

        // 验证每个数据集
        for dataset in &self.datasets {
            dataset.validate()?;
        }

        // 验证全局网络设置
        if let Some(global_settings) = &self.global_network_settings {
            global_settings.validate()?;
        }

        Ok(())
    }
}

/// PPROJ文件管理器
pub struct PprojManager {
    config: Option<PprojConfig>,
}

impl PprojManager {
    /// 创建新的PPROJ管理器
    pub fn new() -> Self {
        Self { config: None }
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

    /// 保存工程配置到PPROJ文件
    pub fn save_config<P: AsRef<Path>>(config: &PprojConfig, pproj_file_path: P) -> Result<()> {
        let xml_content = Self::serialize_to_xml(config)?;
        fs::write(pproj_file_path.as_ref(), xml_content)?;

        info!("PPROJ工程文件已保存: {:?}", pproj_file_path.as_ref());
        Ok(())
    }

    /// 从PPROJ文件加载工程配置
    pub fn load_config<P: AsRef<Path>>(pproj_file_path: P) -> Result<PprojConfig> {
        let xml_content = fs::read_to_string(pproj_file_path.as_ref())?;
        let config = Self::deserialize_from_xml(&xml_content)?;

        // 验证配置
        config.validate()?;

        info!("PPROJ工程文件已加载: {:?}", pproj_file_path.as_ref());
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

    /// 从XML格式反序列化工程配置
    fn deserialize_from_xml(xml_content: &str) -> Result<PprojConfig> {
        let config: PprojConfig = serde_xml_rs::from_str(xml_content)
            .map_err(|e| PlaybackError::FormatError(format!("XML反序列化失败: {}", e)))?;

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

    /// 获取当前配置
    pub fn get_config(&self) -> Option<&PprojConfig> {
        self.config.as_ref()
    }

    /// 设置配置
    pub fn set_config(&mut self, config: PprojConfig) {
        self.config = Some(config);
    }
}

impl Default for PprojManager {
    fn default() -> Self {
        Self::new()
    }
}

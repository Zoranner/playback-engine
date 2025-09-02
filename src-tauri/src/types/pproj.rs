use crate::types::common::PlaybackError;
use serde::{Deserialize, Serialize};

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
            _ => Err(PlaybackError::ParseError(format!("未知的网络类型: {}", s))),
        }
    }
}

/// 网络配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "network_config")]
pub struct NetworkConfig {
    pub network_type: NetworkType,
    pub ip_address: String,
    pub port: u16,
    pub interface: Option<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            network_type: NetworkType::default(),
            ip_address: "224.0.0.1".to_string(),
            port: 8080,
            interface: None,
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
        }
    }

    /// 创建组播配置
    pub fn multicast(ip: &str, port: u16) -> Self {
        Self {
            network_type: NetworkType::Multicast,
            ip_address: ip.to_string(),
            port,
            interface: None,
        }
    }

    /// 创建广播配置
    pub fn broadcast(port: u16) -> Self {
        Self {
            network_type: NetworkType::Broadcast,
            ip_address: "255.255.255.255".to_string(),
            port,
            interface: None,
        }
    }

    /// 验证网络配置
    pub fn validate(&self) -> crate::types::common::Result<()> {
        // 验证IP地址格式
        if let Err(_) = self.ip_address.parse::<std::net::IpAddr>() {
            return Err(PlaybackError::ParseError(format!(
                "无效的IP地址: {}",
                self.ip_address
            )));
        }

        // 验证端口范围
        if self.port == 0 {
            return Err(PlaybackError::ParseError("端口号不能为0".to_string()));
        }

        // 验证组播地址范围
        if self.network_type == NetworkType::Multicast {
            if let Ok(ip) = self.ip_address.parse::<std::net::Ipv4Addr>() {
                if !ip.is_multicast() {
                    return Err(PlaybackError::ParseError(format!(
                        "非组播地址: {}",
                        self.ip_address
                    )));
                }
            }
        }

        Ok(())
    }
}

/// 数据集配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "dataset")]
pub struct DatasetConfig {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub network_config: NetworkConfig,
}

impl DatasetConfig {
    /// 创建新的数据集配置
    pub fn new(name: String, path: impl AsRef<std::path::Path>) -> Self {
        Self {
            name,
            description: None,
            path: path.as_ref().to_string_lossy().to_string(),
            network_config: NetworkConfig::default(),
        }
    }

    /// 设置描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置网络配置
    pub fn with_network_config(mut self, network_config: NetworkConfig) -> Self {
        self.network_config = network_config;
        self
    }

    /// 验证数据集配置
    pub fn validate(&self) -> crate::types::common::Result<()> {
        // 验证路径是否存在
        let path = std::path::Path::new(&self.path);
        if !path.exists() {
            return Err(PlaybackError::ProjectError(format!(
                "数据集路径不存在: {}",
                self.path
            )));
        }

        if !path.is_dir() {
            return Err(PlaybackError::ProjectError(format!(
                "数据集路径不是目录: {}",
                self.path
            )));
        }

        // 验证网络配置
        self.network_config.validate()?;

        Ok(())
    }
}

/// PPROJ工程配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "pproj_config")]
pub struct PprojConfig {
    pub version: String,
    pub name: String,
    pub description: Option<String>,
    pub created_time: String,
    pub modified_time: String,
    #[serde(rename = "dataset")]
    pub datasets: Vec<DatasetConfig>,

    // 兼容字段
    pub project_name: String,
    pub project_description: Option<String>,
    pub author: Option<String>,
    pub tags: Vec<String>,
}

impl PprojConfig {
    /// 创建新的工程配置
    pub fn new(project_name: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            version: "1.0".to_string(),
            name: project_name.clone(),
            description: None,
            created_time: now.clone(),
            modified_time: now,
            datasets: Vec::new(),
            project_name,
            project_description: None,
            author: None,
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
        self.description = Some(description.clone());
        self.project_description = Some(description);
        self
    }

    /// 设置作者
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// 添加标签
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// 验证工程配置
    pub fn validate(&self) -> crate::types::common::Result<()> {
        // 验证数据集名称唯一性
        let mut names = std::collections::HashSet::new();
        for dataset in &self.datasets {
            if !names.insert(&dataset.name) {
                return Err(PlaybackError::ProjectError(format!(
                    "重复的数据集名称: {}",
                    dataset.name
                )));
            }
        }

        // 验证每个数据集
        for dataset in &self.datasets {
            dataset.validate()?;
        }

        Ok(())
    }
}

//! 配置状态管理

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// UDP发送配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UDPConfig {
    pub mode: String, // "broadcast", "multicast", "unicast"
    pub target_ip: String,
    pub target_port: u16,
    pub interface: Option<String>,
}

/// 数据集配置状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfigState {
    pub name: String,
    pub udp_config: UDPConfig,
    pub enabled: bool,
}

/// 配置状态管理器
#[derive(Debug, Clone)]
pub struct ConfigState {
    pub dataset_configs: HashMap<String, DatasetConfigState>,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            dataset_configs: HashMap::new(),
        }
    }

    pub fn set_dataset_config(&mut self, name: String, config: DatasetConfigState) {
        self.dataset_configs.insert(name, config);
    }

    pub fn get_dataset_config(&self, name: &str) -> Option<&DatasetConfigState> {
        self.dataset_configs.get(name)
    }

    pub fn remove_dataset_config(&mut self, name: &str) {
        self.dataset_configs.remove(name);
    }

    pub fn list_dataset_configs(&self) -> Vec<&DatasetConfigState> {
        self.dataset_configs.values().collect()
    }
}

impl Default for ConfigState {
    fn default() -> Self {
        Self::new()
    }
}

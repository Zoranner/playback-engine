//! 配置管理器

use crate::state::config_state::{ConfigState, DatasetConfigState, UDPConfig};
use crate::streaming::udp_sender::{NetworkMode, UDPSender};
use log::info;
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Debug)]
pub struct ConfigManager {
    config: ConfigState,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            config: ConfigState::new(),
        }
    }

    pub fn get_config(&self) -> &ConfigState {
        &self.config
    }

    pub fn update_config(&mut self, new_config: ConfigState) {
        self.config = new_config;
    }

    /// 根据数据集名称创建UDP发送器
    pub fn create_udp_sender_for_dataset(&self, dataset_name: &str) -> Result<UDPSender, String> {
        let config = self
            .config
            .get_dataset_config(dataset_name)
            .ok_or_else(|| format!("数据集配置不存在: {}", dataset_name))?;

        let mode = match config.udp_config.mode.as_str() {
            "broadcast" => NetworkMode::Broadcast,
            "multicast" => {
                let group = std::net::Ipv4Addr::from_str(&config.udp_config.target_ip)
                    .map_err(|_| format!("无效的组播地址: {}", config.udp_config.target_ip))?;
                NetworkMode::Multicast { group }
            }
            "unicast" => {
                let addr: SocketAddr = format!(
                    "{}:{}",
                    config.udp_config.target_ip, config.udp_config.target_port
                )
                .parse()
                .map_err(|_| {
                    format!(
                        "无效的目标地址: {}",
                        format!(
                            "{}:{}",
                            config.udp_config.target_ip, config.udp_config.target_port
                        )
                    )
                })?;
                NetworkMode::Unicast { target: addr }
            }
            _ => return Err(format!("不支持的UDP模式: {}", config.udp_config.mode)),
        };

        let target_addr = format!(
            "{}:{}",
            config.udp_config.target_ip, config.udp_config.target_port
        )
        .parse()
        .map_err(|_| {
            format!(
                "无效的目标地址: {}:{}",
                config.udp_config.target_ip, config.udp_config.target_port
            )
        })?;

        let sender =
            UDPSender::new(mode, target_addr).map_err(|e| format!("创建UDP发送器失败: {:?}", e))?;

        info!(
            "为数据集 '{}' 创建UDP发送器成功: {}",
            dataset_name, target_addr
        );
        Ok(sender)
    }

    /// 更新数据集的UDP配置
    pub fn update_dataset_config(&mut self, dataset_name: String, udp_config: UDPConfig) {
        let config = DatasetConfigState {
            name: dataset_name.clone(),
            udp_config,
            enabled: true,
        };

        self.config.set_dataset_config(dataset_name, config);
    }

    /// 获取所有启用的数据集配置
    pub fn get_enabled_datasets(&self) -> Vec<&DatasetConfigState> {
        self.config
            .list_dataset_configs()
            .into_iter()
            .filter(|config| config.enabled)
            .collect()
    }
}

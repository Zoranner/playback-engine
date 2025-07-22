//! 数据协调器 - 协调PCAP读取和UDP发送

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::playback::scheduler::EventScheduler;
use crate::streaming::udp_sender::UDPSender;
use crate::state::config_state::DatasetConfigState;

#[derive(Debug)]
pub struct DataCoordinator {
    scheduler: Arc<Mutex<EventScheduler>>,
    sender: Arc<Mutex<Option<UDPSender>>>,
}

impl DataCoordinator {
    pub fn new() -> Self {
        Self {
            scheduler: Arc::new(Mutex::new(EventScheduler::new())),
            sender: Arc::new(Mutex::new(None)),
        }
    }

    /// 加载数据集到调度器
    pub async fn load_dataset(&self, _dataset_name: &str, _config: &DatasetConfigState) -> Result<(), String> {
        // TODO: 实现数据集加载逻辑
        Ok(())
    }

    /// 发送当前时间点的数据
    pub async fn send_current_data(&mut self, current_time: u64) -> Result<(), String> {
        let mut scheduler = self.scheduler.lock().await;
        
        while let Some(event) = scheduler.get_next_event(current_time) {
            // 发送事件数据
            if let Some(sender) = &*self.sender.lock().await {
                sender.send_data(&event.data)
                    .map_err(|e| format!("发送失败: {}", e))?;
            }
        }
        
        Ok(())
    }
}
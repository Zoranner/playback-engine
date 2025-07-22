use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use log::{info, debug};

use crate::state::playback_state::{PlaybackState, PlaybackStatus};
use crate::playback::coordinator::DataCoordinator;
use crate::streaming::config_manager::ConfigManager;
use crate::playback::timeline::TimelineController;

/// 回放引擎 - 核心回放控制
#[derive(Debug)]
pub struct PlaybackEngine {
    state: Arc<Mutex<PlaybackState>>,
    coordinator: DataCoordinator,
    config_manager: ConfigManager,
    timeline: Option<TimelineController>,
    is_running: Arc<Mutex<bool>>,
}

impl PlaybackEngine {
    pub fn new(state: Arc<Mutex<PlaybackState>>) -> Self {
        Self {
            state,
            coordinator: DataCoordinator::new(),
            config_manager: ConfigManager::new(),
            timeline: None,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    /// 开始回放
    pub async fn start(
        &mut self,
        dataset_name: String
    ) -> Result<(), String> {
        info!("开始回放数据集: {}", dataset_name);
        
        let mut state = self.state.lock().await;
        state.current_dataset = Some(dataset_name.clone());
        state.status = PlaybackStatus::Playing;
        
        // 加载数据集配置
        let _config = self.config_manager.get_config()
            .get_dataset_config(&dataset_name)
            .ok_or_else(|| format!("数据集配置不存在: {}", dataset_name))?
            .clone();

        // 初始化时间轴
        self.timeline = Some(TimelineController::new(0, 1000)); // 临时时间范围
        
        // 启动回放循环
        self.start_playback_loop(dataset_name).await?;
        
        Ok(())
    }

    /// 暂停回放
    pub async fn pause(
        &mut self
    ) -> Result<(), String> {
        info!("暂停回放");
        
        let mut state = self.state.lock().await;
        state.status = PlaybackStatus::Paused;
        
        let mut is_running = self.is_running.lock().await;
        *is_running = false;
        
        Ok(())
    }

    /// 停止回放
    pub async fn stop(
        &mut self
    ) -> Result<(), String> {
        info!("停止回放");
        
        let mut state = self.state.lock().await;
        state.status = PlaybackStatus::Stopped;
        state.current_timestamp = 0;
        
        let mut is_running = self.is_running.lock().await;
        *is_running = false;
        
        if let Some(timeline) = &mut self.timeline {
            timeline.reset();
        }
        
        Ok(())
    }

    /// 跳转到指定时间点
    pub async fn seek_to(
        &mut self,
        timestamp: u64
    ) -> Result<(), String> {
        info!("跳转到时间戳: {}", timestamp);
        
        if let Some(timeline) = &mut self.timeline {
            timeline.set_current_time(timestamp);
            
            let mut state = self.state.lock().await;
            state.current_timestamp = timestamp;
        }
        
        Ok(())
    }

    /// 设置回放速度
    pub async fn set_speed(
        &mut self,
        speed: f64
    ) -> Result<(), String> {
        info!("设置回放速度: {}", speed);
        
        if let Some(timeline) = &mut self.timeline {
            timeline.set_playback_speed(speed);
            
            let mut state = self.state.lock().await;
            state.playback_speed = speed;
        }
        
        Ok(())
    }

    /// 获取当前状态
    pub async fn get_state(
        &self
    ) -> PlaybackState {
        let state = self.state.lock().await;
        state.clone()
    }

    /// 启动回放循环
    async fn start_playback_loop(
        &self,
        _dataset_name: String
    ) -> Result<(), String> {
        let is_running = self.is_running.clone();
        let state = self.state.clone();
        
        let mut interval = interval(Duration::from_millis(10)); // 10ms间隔
        
        tokio::spawn(async move {
            *is_running.lock().await = true;
            
            while *is_running.lock().await {
                interval.tick().await;
                
                let mut state_guard = state.lock().await;
                if state_guard.status != PlaybackStatus::Playing {
                    continue;
                }
                
                // 更新播放进度
                state_guard.current_timestamp += 1; // 模拟进度
                if state_guard.current_timestamp >= 1000 {
                    state_guard.current_timestamp = 1000;
                    state_guard.status = PlaybackStatus::Stopped;
                    break;
                }
            }
            
            *is_running.lock().await = false;
            debug!("回放循环结束");
        });
        
        Ok(())
    }
}
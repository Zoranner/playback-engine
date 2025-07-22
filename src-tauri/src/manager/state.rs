//! 全局状态管理
//! 
//! 该模块提供全局状态管理功能，用于管理应用运行时的各种状态。

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// 播放状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlaybackState {
    /// 停止状态
    Stopped,
    /// 播放中
    Playing,
    /// 暂停状态
    Paused,
    /// 完成状态
    Completed,
}

/// 全局应用状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// 当前播放状态
    pub playback_state: PlaybackState,
    /// 当前播放的数据集
    pub current_dataset: Option<String>,
    /// 当前播放位置（时间戳）
    pub current_position: u64,
    /// 播放速度
    pub playback_speed: f64,
    /// 是否循环播放
    pub loop_playback: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            playback_state: PlaybackState::Stopped,
            current_dataset: None,
            current_position: 0,
            playback_speed: 1.0,
            loop_playback: false,
        }
    }
}

/// 状态管理器
pub struct StateManager {
    /// 应用状态
    state: Arc<RwLock<AppState>>,
}

impl StateManager {
    /// 创建新的状态管理器
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::default())),
        }
    }

    /// 获取当前状态
    pub async fn get_state(&self) -> AppState {
        let state = self.state.read().await;
        state.clone()
    }

    /// 更新播放状态
    pub async fn update_playback_state(
        &self,
        new_state: PlaybackState,
    ) {
        let mut state = self.state.write().await;
        state.playback_state = new_state;
    }

    /// 更新当前数据集
    pub async fn update_current_dataset(
        &self,
        dataset_name: Option<String>,
    ) {
        let mut state = self.state.write().await;
        state.current_dataset = dataset_name;
    }

    /// 更新播放位置
    pub async fn update_position(&self, position: u64) {
        let mut state = self.state.write().await;
        state.current_position = position;
    }

    /// 更新播放速度
    pub async fn update_speed(&self, speed: f64) {
        let mut state = self.state.write().await;
        state.playback_speed = speed;
    }

    /// 重置状态
    pub async fn reset(&self) {
        let mut state = self.state.write().await;
        *state = AppState::default();
    }
}
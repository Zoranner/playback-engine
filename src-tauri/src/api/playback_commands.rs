use log::info;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

use crate::state::playback_state::PlaybackState;

/// 开始回放
#[tauri::command]
pub async fn start_playback(
    app: AppHandle,
    dataset_name: String,
) -> std::result::Result<(), String> {
    info!("开始回放数据集: {}", dataset_name);

    let state = app.state::<Arc<Mutex<crate::state::app_state::AppState>>>();
    let mut state_guard = state.lock().await;

    state_guard.playback_engine.start(dataset_name).await
}

/// 暂停回放
#[tauri::command]
pub async fn pause_playback(app: AppHandle) -> std::result::Result<(), String> {
    info!("暂停回放");

    let state = app.state::<Arc<Mutex<crate::state::app_state::AppState>>>();
    let mut state_guard = state.lock().await;

    state_guard.playback_engine.pause().await
}

/// 停止回放
#[tauri::command]
pub async fn stop_playback(app: AppHandle) -> std::result::Result<(), String> {
    info!("停止回放");

    let state = app.state::<Arc<Mutex<crate::state::app_state::AppState>>>();
    let mut state_guard = state.lock().await;

    state_guard.playback_engine.stop().await
}

/// 跳转到指定时间
#[tauri::command]
pub async fn seek_to_time(app: AppHandle, timestamp: u64) -> std::result::Result<(), String> {
    info!("跳转到时间戳: {}", timestamp);

    let state = app.state::<Arc<Mutex<crate::state::app_state::AppState>>>();
    let mut state_guard = state.lock().await;

    state_guard.playback_engine.seek_to(timestamp).await
}

/// 设置回放速度
#[tauri::command]
pub async fn set_playback_speed(app: AppHandle, speed: f64) -> std::result::Result<(), String> {
    info!("设置回放速度: {}", speed);

    let state = app.state::<Arc<Mutex<crate::state::app_state::AppState>>>();
    let mut state_guard = state.lock().await;

    state_guard.playback_engine.set_speed(speed).await
}

/// 获取当前回放状态
#[tauri::command]
pub async fn get_playback_state(app: AppHandle) -> std::result::Result<PlaybackState, String> {
    let state = app.state::<Arc<Mutex<crate::state::app_state::AppState>>>();
    let state_guard = state.lock().await;

    Ok(state_guard.playback_engine.get_state().await)
}

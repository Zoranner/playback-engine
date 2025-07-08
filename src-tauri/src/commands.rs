use tauri::State;
use std::sync::Mutex;
use log::{info, error, debug};

use crate::types::{ProjectInfo, PlaybackState};
use crate::project_manager::ProjectManager;

/// 项目管理器状态
type ProjectManagerState = Mutex<ProjectManager>;

/// 播放状态
type PlaybackStateManager = Mutex<PlaybackState>;

/// 打开工程目录
#[tauri::command]
pub async fn open_project(
    path: String,
    project_manager: State<'_, ProjectManagerState>,
) -> Result<ProjectInfo, String> {
    info!("收到打开工程请求: {}", path);
    
    // 创建一个新的ProjectManager实例用于异步操作
    let mut temp_manager = ProjectManager::new();
    
    match temp_manager.open_project(&path).await {
        Ok(project_info) => {
            // 成功后，将结果保存到状态管理器中
            {
                let mut manager = project_manager.lock().map_err(|e| {
                    let msg = format!("获取项目管理器锁失败: {}", e);
                    error!("{}", msg);
                    msg
                })?;
                *manager = temp_manager;
            }
            
            info!("工程打开成功: {}", project_info.name);
            Ok(project_info)
        }
        Err(e) => {
            let msg = format!("打开工程失败: {}", e);
            error!("{}", msg);
            Err(msg)
        }
    }
}

/// 关闭当前工程
#[tauri::command]
pub async fn close_project(
    project_manager: State<'_, ProjectManagerState>,
) -> Result<(), String> {
    info!("收到关闭工程请求");
    
    let mut manager = project_manager.lock().map_err(|e| {
        let msg = format!("获取项目管理器锁失败: {}", e);
        error!("{}", msg);
        msg
    })?;
    
    manager.close_project();
    info!("工程已关闭");
    Ok(())
}

/// 获取当前工程信息
#[tauri::command]
pub async fn get_project_metadata(
    project_manager: State<'_, ProjectManagerState>,
) -> Result<Option<ProjectInfo>, String> {
    debug!("收到获取工程元数据请求");
    
    let manager = project_manager.lock().map_err(|e| {
        let msg = format!("获取项目管理器锁失败: {}", e);
        error!("{}", msg);
        msg
    })?;
    
    let project_info = manager.get_current_project().cloned();
    debug!("返回工程信息: {:?}", project_info.is_some());
    
    Ok(project_info)
}

/// 验证工程目录
#[tauri::command]
pub async fn validate_project_directory(path: String) -> Result<bool, String> {
    debug!("收到验证工程目录请求: {}", path);
    
    match ProjectManager::validate_project_directory(&path) {
        Ok(_) => {
            debug!("工程目录验证成功");
            Ok(true)
        }
        Err(e) => {
            let msg = format!("工程目录验证失败: {}", e);
            debug!("{}", msg);
            Ok(false) // 返回false而不是错误，让前端处理
        }
    }
}

/// 获取播放状态
#[tauri::command]
pub async fn get_playback_state(
    playback_state: State<'_, PlaybackStateManager>,
) -> Result<PlaybackState, String> {
    debug!("收到获取播放状态请求");
    
    let state = playback_state.lock().map_err(|e| {
        let msg = format!("获取播放状态锁失败: {}", e);
        error!("{}", msg);
        msg
    })?;
    
    Ok(state.clone())
}

/// 重置播放状态
#[tauri::command]
pub async fn reset_playback_state(
    playback_state: State<'_, PlaybackStateManager>,
) -> Result<(), String> {
    info!("收到重置播放状态请求");
    
    let mut state = playback_state.lock().map_err(|e| {
        let msg = format!("获取播放状态锁失败: {}", e);
        error!("{}", msg);
        msg
    })?;
    
    *state = PlaybackState::default();
    info!("播放状态已重置");
    Ok(())
}

/// 测试命令 - 用于验证前后端通信
#[tauri::command]
pub async fn test_connection() -> Result<String, String> {
    debug!("收到测试连接请求");
    Ok("后端连接正常".to_string())
}

/// 获取应用信息
#[tauri::command]
pub async fn get_app_info() -> Result<serde_json::Value, String> {
    debug!("收到获取应用信息请求");
    
    let app_info = serde_json::json!({
        "name": "Playback Engine",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "综合数据回放复盘软件",
        "rust_version": env!("CARGO_PKG_RUST_VERSION"),
    });
    
    Ok(app_info)
}

// 注意: 播放控制相关的命令将在播放引擎模块完成后添加
// 包括: start_playback, pause_playback, stop_playback, seek_to_time, set_playback_speed

/// 获取所有可用的命令列表（用于调试）
#[tauri::command]
pub async fn get_available_commands() -> Result<Vec<String>, String> {
    let commands = vec![
        "open_project".to_string(),
        "close_project".to_string(),
        "get_project_metadata".to_string(),
        "validate_project_directory".to_string(),
        "get_playback_state".to_string(),
        "reset_playback_state".to_string(),
        "test_connection".to_string(),
        "get_app_info".to_string(),
        "get_available_commands".to_string(),
    ];
    
    Ok(commands)
} 
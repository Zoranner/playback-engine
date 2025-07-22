use tauri::{AppHandle, Manager};
use log::{info, error};

use crate::project::loader::ProjectLoader;
use crate::types::ProjectInfo;
use crate::state::app_state::AppState;

/// 打开工程目录
#[tauri::command]
pub async fn open_project(
    app: AppHandle,
    project_path: String,
) -> std::result::Result<ProjectInfo, String> {
    info!("接收到打开工程请求: {}", project_path);

    let mut loader = ProjectLoader::new();
    match loader.open_project(&project_path).await {
        Ok(project_info) => {
            // 保存到状态管理器
            let state = app.state::<std::sync::Mutex<AppState>>();
            {
                let mut state_guard = state.lock().unwrap();
                state_guard.set_current_project(Some(project_info.clone()));
            }

            info!("工程打开成功: {}", project_info.name);
            Ok(project_info)
        }
        Err(e) => {
            error!("打开工程失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 获取当前工程信息
#[tauri::command]
pub fn get_project_info(
    app: AppHandle,
) -> std::result::Result<Option<ProjectInfo>, String> {
    let state = app.state::<std::sync::Mutex<AppState>>();
    let state_guard = state.lock().unwrap();
    Ok(state_guard.current_project())
}

/// 关闭当前工程
#[tauri::command]
pub fn close_project(
    app: AppHandle,
) -> std::result::Result<(), String> {
    let state = app.state::<std::sync::Mutex<AppState>>();
    {
        let mut state_guard = state.lock().unwrap();
        state_guard.set_current_project(None);
    }
    info!("工程已关闭");
    Ok(())
}
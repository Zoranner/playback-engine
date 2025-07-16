use tauri::{AppHandle, Manager};
use log::{info, error};

use crate::manager::ProjectManager;
use crate::types::ProjectInfo;
use crate::pidx::reader::IndexStats;

/// 打开工程目录
#[tauri::command]
pub async fn open_project(
    app: AppHandle,
    project_path: String,
) -> std::result::Result<ProjectInfo, String> {
    info!("接收到打开工程请求: {}", project_path);

    // 创建临时管理器来执行异步操作
    let mut temp_manager = ProjectManager::new();

    match temp_manager.open_project(&project_path).await {
        Ok(project_info) => {
            // 成功后将结果保存到状态管理器
            let project_manager_state = app.state::<std::sync::Mutex<ProjectManager>>();
            {
                let mut manager = project_manager_state.lock().unwrap();
                *manager = temp_manager;
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
    let project_manager_state = app.state::<std::sync::Mutex<ProjectManager>>();
    let manager = project_manager_state.lock().unwrap();

    match manager.get_current_project() {
        Some(project) => Ok(Some(project.clone())),
        None => Ok(None)
    }
}

/// 列出所有数据集
#[tauri::command]
pub fn list_datasets(
    app: AppHandle,
) -> std::result::Result<Vec<String>, String> {
    let project_manager_state = app.state::<std::sync::Mutex<ProjectManager>>();
    let manager = project_manager_state.lock().unwrap();
    Ok(manager.list_dataset_names())
}

/// 获取数据集统计信息
#[tauri::command]
pub fn get_dataset_stats(
    app: AppHandle,
    dataset_name: String,
) -> std::result::Result<Option<IndexStats>, String> {
    let project_manager_state = app.state::<std::sync::Mutex<ProjectManager>>();
    let manager = project_manager_state.lock().unwrap();

    match manager.get_dataset_reader(&dataset_name) {
        Some(reader) => {
            let stats = crate::pidx::reader::PidxReader::get_index_stats(&reader.index);
            Ok(Some(stats))
        }
        None => Ok(None)
    }
}

/// 关闭当前工程
#[tauri::command]
pub fn close_project(
    app: AppHandle,
) -> std::result::Result<(), String> {
    let project_manager_state = app.state::<std::sync::Mutex<ProjectManager>>();
    let mut manager = project_manager_state.lock().unwrap();
    manager.close_project();
    info!("工程已关闭");
    Ok(())
}

use tauri::{AppHandle, Manager};
use serde_json;

use crate::manager::ProjectManager;

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
) -> std::result::Result<Option<u64>, String> {
    let project_manager_state = app.state::<std::sync::Mutex<ProjectManager>>();
    let manager = project_manager_state.lock().unwrap();

    match manager.get_dataset_reader(&dataset_name) {
        Some(reader) => Ok(Some(reader.total_packets)),
        None => Ok(None)
    }
}

/// 获取数据集详细信息
#[tauri::command]
pub fn get_dataset_info(
    app: AppHandle,
    dataset_name: String,
) -> std::result::Result<Option<serde_json::Value>, String> {
    let project_manager_state = app.state::<std::sync::Mutex<ProjectManager>>();
    let manager = project_manager_state.lock().unwrap();

    match manager.get_dataset_reader(&dataset_name) {
        Some(reader) => {
            let info = serde_json::json!({
                "name": dataset_name,
                "config": reader.config,
                "pcap_files": reader.pcap_files.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>(),
                "total_packets": reader.total_packets,
            });
            Ok(Some(info))
        },
        None => Ok(None)
    }
}
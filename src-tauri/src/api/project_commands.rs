use tauri::{AppHandle, Manager};
use log::{info, error};
use serde_json::json;

use crate::project::loader::ProjectLoader;
use crate::project::structure::ProjectStructure;
use crate::state::app_state::AppState;
use crate::types::ProjectInfo;

/// 选择项目目录
#[tauri::command]
pub async fn select_project_directory(
    app: AppHandle,
) -> std::result::Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    use tokio::sync::oneshot;

    let (tx, rx) = oneshot::channel();

    let dialog = app.dialog().file();
    dialog.pick_folder(move |folder_path| {
        match folder_path {
            Some(path) => {
                info!("用户选择了目录: {:?}", path);
                let _ = tx.send(Some(path.to_string()));
            }
            None => {
                info!("用户取消了选择");
                let _ = tx.send(None);
            }
        }
    });

    // 等待用户选择结果
    match rx.await {
        Ok(result) => Ok(result),
        Err(_) => Err("对话框操作失败".to_string()),
    }
}

/// 打开工程目录
#[tauri::command]
pub async fn open_project(
    app: AppHandle,
    path: String,
) -> std::result::Result<ProjectInfo, String> {
    info!("接收到打开工程请求: {}", path);

    let mut loader = ProjectLoader::new();
    match loader.open_project(&path).await {
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

/// 获取项目结构信息
#[tauri::command]
pub async fn get_project_structure(
    project_path: String,
) -> std::result::Result<serde_json::Value, String> {
    info!("获取项目结构信息: {}", project_path);

    match ProjectStructure::from_path(&project_path) {
        Ok(structure) => {
            // 转换为前端需要的数据格式
            let datasets: Vec<serde_json::Value> = structure.datasets
                .into_iter()
                .map(|dataset| {
                    let mut all_files: Vec<serde_json::Value> = Vec::new();

                    // 添加PCAP文件
                    for path in dataset.pcap_files {
                        let metadata = std::fs::metadata(&path);
                        let size = metadata.map(|m| m.len()).unwrap_or(0);
                        all_files.push(json!({
                            "name": path.file_name().unwrap().to_string_lossy(),
                            "path": path.to_string_lossy(),
                            "size": size,
                            "type": "pcap"
                        }));
                    }

                    // 添加索引文件
                    for path in dataset.index_files {
                        let metadata = std::fs::metadata(&path);
                        let size = metadata.map(|m| m.len()).unwrap_or(0);
                        all_files.push(json!({
                            "name": path.file_name().unwrap().to_string_lossy(),
                            "path": path.to_string_lossy(),
                            "size": size,
                            "type": "pidx"
                        }));
                    }

                    // 按文件名排序
                    all_files.sort_by(|a, b| {
                        a["name"].as_str().unwrap_or("").cmp(b["name"].as_str().unwrap_or(""))
                    });

                    json!({
                        "name": dataset.name,
                        "path": dataset.path.to_string_lossy(),
                        "files": all_files
                    })
                })
                .collect();

            Ok(json!({
                "project_name": structure.name,
                "project_path": structure.root_path.to_string_lossy(),
                "datasets": datasets
            }))
        }
        Err(e) => {
            error!("获取项目结构失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 创建数据集
#[tauri::command]
pub async fn create_dataset(
    project_path: String,
    dataset_name: String,
) -> std::result::Result<serde_json::Value, String> {
    use std::fs;
    use std::path::Path;

    info!("创建数据集: {} in project {}", dataset_name, project_path);

    // 验证数据集名称
    if dataset_name.is_empty() || dataset_name.len() > 50 {
        return Err("数据集名称无效".to_string());
    }

    // 检查数据集名称是否包含非法字符
    if dataset_name.contains('/') || dataset_name.contains('\\') || dataset_name.contains(':') {
        return Err("数据集名称包含非法字符".to_string());
    }

    let dataset_path = Path::new(&project_path).join(&dataset_name);

    // 检查数据集是否已存在
    if dataset_path.exists() {
        return Err("数据集已存在".to_string());
    }

    // 创建数据集目录
    match fs::create_dir(&dataset_path) {
        Ok(_) => {
            info!("数据集目录创建成功: {:?}", dataset_path);
            Ok(json!({
                "success": true,
                "message": "数据集创建成功",
                "dataset_name": dataset_name,
                "dataset_path": dataset_path.to_string_lossy()
            }))
        }
        Err(e) => {
            error!("创建数据集目录失败: {}", e);
            Err(format!("创建数据集目录失败: {}", e))
        }
    }
}

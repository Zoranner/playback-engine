//! 系统操作相关命令
//! 
//! 该模块提供系统级别的操作命令，如文件操作、系统信息等。

use tauri::AppHandle;
use log::info;

/// 获取系统信息
#[tauri::command]
pub fn get_system_info() -> std::result::Result<serde_json::Value, String> {
    let info = serde_json::json!({
        "platform": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "version": env!("CARGO_PKG_VERSION"),
    });
    
    info!("获取系统信息: {}", info);
    Ok(info)
}

/// 选择目录
#[tauri::command]
pub async fn select_directory(
    _app: AppHandle,
) -> std::result::Result<Option<String>, String> {
    info!("选择目录功能暂未实现");
    Ok(None)
}

/// 检查文件是否存在
#[tauri::command]
pub fn check_file_exists(file_path: String) -> std::result::Result<bool, String> {
    let exists = std::path::Path::new(&file_path).exists();
    info!("检查文件是否存在: {} -> {}", file_path, exists);
    Ok(exists)
}
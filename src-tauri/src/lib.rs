use std::sync::Mutex;
use log::info;

// 模块声明
mod types;
mod pcap_reader;
mod project_manager;
mod commands;
mod pidx;
mod pproj;
mod multi_pcap_reader;

// 重新导出类型
pub use types::*;
pub use pcap_reader::PcapReader;
pub use project_manager::ProjectManager;
pub use pidx::{PidxManager, PidxIndex, PacketIndexEntry, PcapFileIndex};
pub use pproj::{PprojManager, PprojConfig, DatasetConfig, NetworkConfig, NetworkType};
pub use multi_pcap_reader::MultiPcapReader;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    env_logger::init();
    info!("启动回放引擎应用");

    // 创建应用状态
    let project_manager = Mutex::new(ProjectManager::new());
    let playback_state = Mutex::new(PlaybackState::default());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(project_manager)
        .manage(playback_state)
        .invoke_handler(tauri::generate_handler![
            commands::open_project,
            commands::close_project,
            commands::get_project_metadata,
            commands::validate_project_directory,
            commands::get_playback_state,
            commands::reset_playback_state,
            commands::test_connection,
            commands::get_app_info,
            commands::get_available_commands,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

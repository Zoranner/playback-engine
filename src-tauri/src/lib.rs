// 模块声明
pub mod types;
pub mod pproj;
pub mod manager;
pub mod commands;

// 重新导出 pcap-io 库的核心类型
pub use pcap_io::{
    Configuration as PcapConfiguration, DataPacket, DataPacketHeader, PcapFileHeader, Read,
    Reader as PcapReader, Write, Writer as PcapWriter,
    // 索引相关类型
    PacketIndexEntry, PcapFileIndex, PidxIndex, PidxReader, PidxWriter,
};

// 重新导出应用类型
pub use manager::ProjectManager;
pub use pproj::{PprojReader, PprojWriter};
pub use types::{AppDataPacket, PacketType, PlaybackError, Result};

// Tauri相关导入
use tauri::Manager;

/// 初始化Tauri应用
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::open_project,
            commands::get_project_info,
            commands::list_datasets,
            commands::get_dataset_stats,
            commands::close_project,
        ])
        .setup(|app| {
            // 初始化日志
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Info)
                .init();

            // 初始化项目管理器
            let project_manager = std::sync::Mutex::new(ProjectManager::new());
            app.manage(project_manager);

            log::info!("Tauri应用初始化完成，集成pcap-io库");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("运行Tauri应用时出错");
}

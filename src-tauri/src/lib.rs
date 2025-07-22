// 模块声明
pub mod api;
pub mod geo;
pub mod playback;
pub mod project;
pub mod state;
pub mod streaming;
pub mod types;

// 重新导出 pcap-io 库的核心类型
pub use pcap_io::{
    Configuration as PcapConfiguration,
    DataPacket,
    DataPacketHeader,
    // 索引相关类型
    PacketIndexEntry,
    PcapFileHeader,
    PcapFileIndex,
    PidxIndex,
    PidxReader,
    PidxWriter,
    Read,
    Reader as PcapReader,
    Write,
    Writer as PcapWriter,
};

// 重新导出应用类型
pub use project::loader::ProjectLoader;
pub use state::app_state::AppState;
pub use state::playback_state::PlaybackState;
pub use types::{AppDataPacket, PacketType, PlaybackError, Result};

// Tauri相关导入
use dotenvy::dotenv;
use std::sync::Arc;
use tauri::Manager;
use tauri::{webview::WebviewWindowBuilder, WebviewUrl};
use tokio::sync::Mutex;

/// 通过环境变量获取前端URL
fn get_frontend_url() -> String {
    let port = std::env::var("TAURI_FRONTEND_PORT").unwrap_or_else(|_| "32030".to_string());
    format!("http://localhost:{}", port)
}

/// 初始化Tauri应用
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 加载 .env 文件
    dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            api::project_commands::select_project_directory,
            api::project_commands::open_project,
            api::project_commands::get_project_info,
            api::project_commands::close_project,
            api::project_commands::get_project_structure,
            api::config_commands::list_datasets,
            api::config_commands::get_dataset_stats,
            api::config_commands::get_dataset_info,
            api::playback_commands::start_playback,
            api::playback_commands::pause_playback,
            api::playback_commands::stop_playback,
            api::playback_commands::seek_to_time,
            api::playback_commands::set_playback_speed,
            api::playback_commands::get_playback_state,
            api::geo_commands::get_map_tile,
            api::geo_commands::get_geojson_data,
            api::geo_commands::get_mvt_tile,
        ])
        .setup(|app| {
            // 初始化日志
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Info)
                .init();

            // 初始化应用状态
            let app_state = std::sync::Mutex::new(AppState::new());
            app.manage(app_state);

            // 动态创建窗口
            let frontend_url = get_frontend_url().parse().unwrap();
            log::info!("Load frontend from: {}", frontend_url);
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(frontend_url))
                .title("综合复盘软件")
                .min_inner_size(960.0, 600.0)
                .maximized(true)
                .build()
                .expect("创建窗口失败");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("运行应用时出错");
}

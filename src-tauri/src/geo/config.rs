//! 瓦片服务配置管理

use crate::types::TileProxyConfig;
use std::env;
use std::path::PathBuf;

/// 获取用户数据目录
fn get_user_data_dir() -> PathBuf {
    // 使用用户主目录下的 .pabae 目录
    if let Some(home_dir) = dirs::home_dir() {
        home_dir.join(".pabae")
    } else {
        // 如果获取不到用户目录，使用当前目录下的 .pabae 文件夹
        PathBuf::from("./.pabae")
    }
}

/// 获取瓦片服务配置
pub fn get_tile_config() -> TileProxyConfig {
    let tile_data_dir = get_user_data_dir()
        .join("tiles")
        .to_string_lossy()
        .to_string();

    TileProxyConfig {
        port: env::var("TILE_SERVICE_PORT")
            .and_then(|s| s.parse().map_err(|_| env::VarError::NotPresent))
            .unwrap_or(32031),
        upstream_url: env::var("UPSTREAM_TILE_URL")
            .unwrap_or_else(|_| "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}".to_string()),
        cache_dir: tile_data_dir,
        request_timeout: env::var("TILE_REQUEST_TIMEOUT")
            .and_then(|s| s.parse().map_err(|_| env::VarError::NotPresent))
            .unwrap_or(30),
    }
}

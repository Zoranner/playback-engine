//! 瓦片服务配置管理

use crate::types::TileProxyConfig;
use std::env;
use std::path::PathBuf;

/// 获取用户缓存目录
fn get_user_cache_dir() -> PathBuf {
    let home = env::var("HOME").expect("无法获取 HOME 环境变量");
    PathBuf::from(home).join(".cache").join("playback-engine")
}

/// 获取瓦片服务配置
pub fn get_tile_config() -> TileProxyConfig {
    let cache_dir = get_user_cache_dir()
        .join("tile_cache")
        .to_string_lossy()
        .to_string();

    TileProxyConfig {
        port: env::var("TILE_SERVICE_PORT")
            .and_then(|s| s.parse().map_err(|_| env::VarError::NotPresent))
            .unwrap_or(8080),
        upstream_url: env::var("UPSTREAM_TILE_URL")
            .unwrap_or_else(|_| "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}".to_string()),
        cache_dir,
        request_timeout: env::var("TILE_REQUEST_TIMEOUT")
            .and_then(|s| s.parse().map_err(|_| env::VarError::NotPresent))
            .unwrap_or(30),
    }
}

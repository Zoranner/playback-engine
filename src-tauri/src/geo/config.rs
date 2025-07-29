//! 瓦片服务配置管理

use crate::types::TileProxyConfig;
use std::env;

/// 获取瓦片服务配置
pub fn get_tile_config() -> TileProxyConfig {
    TileProxyConfig {
      port: env::var("TILE_SERVICE_PORT")
          .unwrap_or_else(|_| "8080".to_string())
          .parse()
          .unwrap_or(8080),
        upstream_url: env::var("UPSTREAM_TILE_URL")
            .unwrap_or_else(|_| "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}".to_string()),
        cache_dir: env::var("TILE_CACHE_DIR")
            .unwrap_or_else(|_| "./tile_cache".to_string()),
        cache_ttl: env::var("TILE_CACHE_TTL")
            .unwrap_or_else(|_| "86400".to_string())
            .parse()
            .unwrap_or(86400),
        request_timeout: env::var("TILE_REQUEST_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30),
    }
}

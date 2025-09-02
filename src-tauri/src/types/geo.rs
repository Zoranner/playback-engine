//! 地理数据类型定义
//!
//! 包含地图瓦片代理服务相关的类型定义

use serde::{Deserialize, Serialize};

/// 瓦片坐标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileCoord {
    /// 瓦片X坐标
    pub x: u32,
    /// 瓦片Y坐标
    pub y: u32,
    /// 缩放级别
    pub z: u8,
}

impl TileCoord {
    /// 创建新的瓦片坐标
    pub fn new(x: u32, y: u32, z: u8) -> Self {
        Self { x, y, z }
    }

    /// 生成瓦片文件名
    pub fn filename(&self) -> String {
        format!("{}_{}_{}.png", self.z, self.x, self.y)
    }

    /// 生成瓦片路径
    pub fn path(&self, cache_dir: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(cache_dir)
            .join(format!("{}", self.z))
            .join(format!("{}", self.x))
            .join(format!("{}.png", self.y))
    }
}

/// 瓦片代理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileProxyConfig {
    /// 服务端口
    pub port: u16,
    /// 上游瓦片服务URL模板
    pub upstream_url: String,
    /// 本地缓存目录
    pub cache_dir: String,
    /// 请求超时时间（秒）
    pub request_timeout: u64,
}

impl Default for TileProxyConfig {
    fn default() -> Self {
        Self {
            upstream_url: "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}".to_string(),
            cache_dir: "./tile_cache".to_string(),
            request_timeout: 30,
            port: 8080,
        }
    }
}

/// 瓦片代理统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileProxyStats {
    /// 缓存命中次数
    pub cache_hits: u64,
    /// 上游请求次数
    pub upstream_requests: u64,
    /// 总请求次数
    pub total_requests: u64,
    /// 缓存大小（字节）
    pub cache_size: u64,
    /// 最后更新时间
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

//! 缓存管理器
//! 
//! 该模块提供数据缓存功能，用于提高PCAP数据读取性能。

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 缓存管理器
pub struct CacheManager {
    /// 数据包缓存
    packet_cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    /// 索引缓存
    index_cache: Arc<RwLock<HashMap<String, Vec<u64>>>>,
}

impl CacheManager {
    /// 创建新的缓存管理器
    pub fn new() -> Self {
        Self {
            packet_cache: Arc::new(RwLock::new(HashMap::new())),
            index_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 获取数据包缓存
    pub async fn get_packet_cache(
        &self,
        key: &str,
    ) -> Option<Vec<u8>> {
        let cache = self.packet_cache.read().await;
        cache.get(key).cloned()
    }

    /// 设置数据包缓存
    pub async fn set_packet_cache(
        &self,
        key: String,
        data: Vec<u8>,
    ) {
        let mut cache = self.packet_cache.write().await;
        cache.insert(key, data);
    }

    /// 清除缓存
    pub async fn clear_cache(&self) {
        let mut packet_cache = self.packet_cache.write().await;
        let mut index_cache = self.index_cache.write().await;
        packet_cache.clear();
        index_cache.clear();
    }
}
//! GeoJSON数据服务

use serde_json::Value;

pub struct GeoJSONService;

impl GeoJSONService {
    pub fn get_data(_dataset_name: &str) -> Value {
        // 实现GeoJSON数据获取
        serde_json::json!({})
    }
}
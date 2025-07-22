//! 测试发送器

use crate::state::config_state::DatasetConfigState;

pub struct TestSender;

impl TestSender {
    pub fn test_dataset(
        _dataset_name: &str,
        _config: &DatasetConfigState,
    ) -> Result<(), String> {
        // 测试发送逻辑
        Ok(())
    }
}
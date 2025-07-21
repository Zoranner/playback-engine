use chrono::{DateTime, Timelike, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

/// 缓存统计信息（内部使用）
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// 缓存条目总数
    pub total_entries: usize,
    /// 缓存命中次数
    pub hit_count: u64,
    /// 缓存未命中次数
    pub miss_count: u64,
    /// 缓存命中率
    pub hit_rate: f64,
}

impl CacheStats {
    /// 创建新的缓存统计信息
    pub fn new() -> Self {
        Self {
            total_entries: 0,
            hit_count: 0,
            miss_count: 0,
            hit_rate: 0.0,
        }
    }

    /// 更新命中率
    pub fn update_hit_rate(&mut self) {
        let total_requests = self.hit_count + self.miss_count;
        self.hit_rate = if total_requests > 0 {
            self.hit_count as f64 / total_requests as f64
        } else {
            0.0
        };
    }
}

/// 字节数组扩展方法
pub trait ByteArrayExtensions {
    /// 获取字节数组的子数组
    fn sub_array(&self, start_index: usize, length: usize) -> Result<Vec<u8>, String>;

    /// 将字节数组转换为十六进制字符串
    fn to_hex_string(&self, separator: &str) -> String;

    /// 将字节数组转换为Base64字符串
    fn to_base64_string(&self) -> String;

    /// 将字节数组转换为UTF8字符串
    fn to_utf8_string(&self) -> Result<String, String>;

    /// 比较两个字节数组是否相等
    fn equals(&self, other: &[u8]) -> bool;

    /// 计算字节数组的哈希值
    fn get_hash_code(&self) -> u32;
}

impl ByteArrayExtensions for [u8] {
    fn sub_array(&self, start_index: usize, length: usize) -> Result<Vec<u8>, String> {
        if start_index >= self.len() {
            return Err("起始索引超出范围".to_string());
        }

        if start_index + length > self.len() {
            return Err("长度超出范围".to_string());
        }

        Ok(self[start_index..start_index + length].to_vec())
    }

    fn to_hex_string(&self, separator: &str) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(self.len() * (2 + separator.len()));
        for (i, &byte) in self.iter().enumerate() {
            if i > 0 && !separator.is_empty() {
                result.push_str(separator);
            }
            result.push_str(&format!("{:02X}", byte));
        }
        result
    }

    fn to_base64_string(&self) -> String {
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, self)
    }

    fn to_utf8_string(&self) -> Result<String, String> {
        String::from_utf8(self.to_vec()).map_err(|e| format!("UTF8解码失败: {}", e))
    }

    fn equals(&self, other: &[u8]) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }

    fn get_hash_code(&self) -> u32 {
        let mut hash = 17u32;
        for &byte in self {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }
}

/// DateTime扩展方法
pub trait DateTimeExtensions {
    /// 将 DateTime 转换为 Unix 时间戳(毫秒)
    fn to_unix_time_milliseconds(&self) -> i64;

    /// 将 Unix 时间戳(毫秒)转换为 DateTime
    fn from_unix_time_milliseconds(milliseconds: i64) -> DateTime<Utc>;

    /// 将 DateTime 转换为 Unix 时间戳(秒)
    fn to_unix_time_seconds(&self) -> u32;

    /// 获取 DateTime 的纳秒部分
    fn get_nanoseconds(&self) -> u32;

    /// 将 Unix 时间戳(秒)和纳秒部分转换为 DateTime
    fn from_unix_time_with_nanoseconds(seconds: u32, nanoseconds: u32) -> DateTime<Utc>;
}

impl DateTimeExtensions for DateTime<Utc> {
    fn to_unix_time_milliseconds(&self) -> i64 {
        self.timestamp_millis()
    }

    fn from_unix_time_milliseconds(milliseconds: i64) -> DateTime<Utc> {
        DateTime::from_timestamp_millis(milliseconds).unwrap_or_default()
    }

    fn to_unix_time_seconds(&self) -> u32 {
        self.timestamp() as u32
    }

    fn get_nanoseconds(&self) -> u32 {
        self.nanosecond()
    }

    fn from_unix_time_with_nanoseconds(seconds: u32, nanoseconds: u32) -> DateTime<Utc> {
        DateTime::from_timestamp(seconds as i64, nanoseconds).unwrap_or_default()
    }
}

/// 计算CRC32校验和
pub fn calculate_crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;

    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }

    !crc
}

/// 二进制转换工具
pub mod binary_converter {
    /// 从字节数组读取小端序整数
    pub fn read_le_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
        if offset + 4 > bytes.len() {
            return Err("字节数组长度不足".to_string());
        }

        Ok(u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]))
    }

    /// 从字节数组读取小端序16位整数
    pub fn read_le_u16(bytes: &[u8], offset: usize) -> Result<u16, String> {
        if offset + 2 > bytes.len() {
            return Err("字节数组长度不足".to_string());
        }

        Ok(u16::from_le_bytes([bytes[offset], bytes[offset + 1]]))
    }

    /// 将整数写入字节数组（小端序）
    pub fn write_le_u32(bytes: &mut [u8], offset: usize, value: u32) -> Result<(), String> {
        if offset + 4 > bytes.len() {
            return Err("字节数组长度不足".to_string());
        }

        let value_bytes = value.to_le_bytes();
        bytes[offset..offset + 4].copy_from_slice(&value_bytes);
        Ok(())
    }

    /// 将16位整数写入字节数组（小端序）
    pub fn write_le_u16(bytes: &mut [u8], offset: usize, value: u16) -> Result<(), String> {
        if offset + 2 > bytes.len() {
            return Err("字节数组长度不足".to_string());
        }

        let value_bytes = value.to_le_bytes();
        bytes[offset..offset + 2].copy_from_slice(&value_bytes);
        Ok(())
    }

    /// 将字符串转换为UTF8字节数组
    pub fn string_to_utf8_bytes(s: &str) -> Vec<u8> {
        s.as_bytes().to_vec()
    }

    /// 从UTF8字节数组转换为字符串
    pub fn utf8_bytes_to_string(bytes: &[u8]) -> Result<String, String> {
        String::from_utf8(bytes.to_vec()).map_err(|e| format!("UTF8解码失败: {}", e))
    }

    /// 将字节数组转换为Base64字符串
    pub fn bytes_to_base64(bytes: &[u8]) -> String {
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
    }

    /// 从Base64字符串转换为字节数组
    pub fn base64_to_bytes(base64_str: &str) -> Result<Vec<u8>, String> {
        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, base64_str)
            .map_err(|e| format!("Base64解码失败: {}", e))
    }
}

/// 文件信息缓存项
#[derive(Debug, Clone)]
pub struct FileInfoCacheItem {
    pub file_info: crate::structures::FileInfo,
    pub cache_time: SystemTime,
}

impl FileInfoCacheItem {
    pub fn new(file_info: crate::structures::FileInfo) -> Self {
        Self {
            file_info,
            cache_time: SystemTime::now(),
        }
    }

    pub fn is_valid(&self, current_file_size: u64, current_write_time: SystemTime) -> bool {
        self.file_info.file_size == current_file_size
            && self.file_info.modified_time
                == current_write_time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string()
    }

    pub fn is_expired(&self, expiration_duration: Duration) -> bool {
        SystemTime::now()
            .duration_since(self.cache_time)
            .unwrap_or(Duration::ZERO)
            >= expiration_duration
    }
}

/// 缓存统计信息
#[derive(Debug, Clone)]
pub struct CacheStatistics {
    pub total_entries: usize,
    pub max_entries: usize,
    pub expired_entries: usize,
    pub last_cleanup_time: SystemTime,
}

impl CacheStatistics {
    pub fn usage_percentage(&self) -> f64 {
        if self.max_entries == 0 {
            0.0
        } else {
            (self.total_entries as f64 / self.max_entries as f64) * 100.0
        }
    }
}

/// 文件信息缓存
pub struct FileInfoCache {
    cache: Arc<Mutex<HashMap<String, FileInfoCacheItem>>>,
    max_entries: usize,
    cache_expiration: Duration,
    cleanup_interval: Duration,
    last_cleanup: Arc<Mutex<SystemTime>>,
    hit_count: Arc<Mutex<u64>>,
    miss_count: Arc<Mutex<u64>>,
}

impl FileInfoCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_entries,
            cache_expiration: Duration::from_secs(30 * 60), // 30分钟
            cleanup_interval: Duration::from_secs(10 * 60), // 10分钟
            last_cleanup: Arc::new(Mutex::new(SystemTime::now())),
            hit_count: Arc::new(Mutex::new(0)),
            miss_count: Arc::new(Mutex::new(0)),
        }
    }

    /// 从缓存中获取文件信息
    pub fn get<P: AsRef<std::path::Path>>(
        &self,
        file_path: P,
    ) -> Option<crate::structures::FileInfo> {
        let path_str = file_path.as_ref().to_string_lossy().to_string();
        let mut cache = self.cache.lock().ok()?;

        // 执行定期清理
        let _ = self.perform_periodic_cleanup(&mut cache);

        if let Some(item) = cache.get(&path_str) {
            // 检查文件是否已修改
            if let Ok(metadata) = std::fs::metadata(&file_path) {
                if let Ok(modified_time) = metadata.modified() {
                    if item.is_valid(metadata.len(), modified_time) {
                        // 缓存命中
                        if let Ok(mut hit_count) = self.hit_count.lock() {
                            *hit_count += 1;
                        }
                        return Some(item.file_info.clone());
                    }
                }
            }
        }

        // 缓存未命中
        if let Ok(mut miss_count) = self.miss_count.lock() {
            *miss_count += 1;
        }

        None
    }

    /// 向缓存中插入文件信息
    pub fn insert<P: AsRef<std::path::Path>>(
        &self,
        file_path: P,
        file_info: crate::structures::FileInfo,
    ) {
        let path_str = file_path.as_ref().to_string_lossy().to_string();

        if let Ok(mut cache) = self.cache.lock() {
            let item = FileInfoCacheItem::new(file_info);
            cache.insert(path_str, item);

            // 检查缓存大小限制
            if cache.len() > self.max_entries {
                let _ = self.cleanup_expired_entries(&mut cache);

                // 如果清理后仍然超过限制，移除最旧的条目
                if cache.len() > self.max_entries {
                    let oldest_key = cache
                        .iter()
                        .min_by_key(|(_, item)| item.cache_time)
                        .map(|(key, _)| key.clone());

                    if let Some(key) = oldest_key {
                        cache.remove(&key);
                    }
                }
            }
        }
    }

    /// 获取或计算文件的数据包数量
    pub fn get_packet_count(&self, file_path: &str) -> Result<u64, String> {
        let path = std::path::Path::new(file_path);

        // 尝试从缓存获取
        if let Some(file_info) = self.get(path) {
            return Ok(file_info.packet_count);
        }

        // 缓存未命中，计算数据包数量
        let packet_count = self.calculate_packet_count(file_path)?;

        // 创建文件信息并插入缓存
        let mut file_info = crate::structures::FileInfo::from_file(path)
            .map_err(|e| format!("创建文件信息失败: {}", e))?;
        file_info.packet_count = packet_count;

        self.insert(path, file_info);
        Ok(packet_count)
    }

    /// 获取缓存统计信息
    pub fn get_cache_stats(&self) -> CacheStats {
        let total_entries = self.cache.lock().map(|cache| cache.len()).unwrap_or(0);

        let hit_count = self.hit_count.lock().map(|guard| *guard).unwrap_or(0);
        let miss_count = self.miss_count.lock().map(|guard| *guard).unwrap_or(0);

        let mut stats = CacheStats {
            total_entries,
            hit_count,
            miss_count,
            hit_rate: 0.0,
        };

        stats.update_hit_rate();
        stats
    }

    fn calculate_packet_count(&self, file_path: &str) -> Result<u64, String> {
        use std::fs::File;
        use std::io::{BufReader, Read, Seek, SeekFrom};

        let file = File::open(file_path).map_err(|e| format!("无法打开文件: {}", e))?;

        let mut reader = BufReader::new(file);
        let file_size = reader
            .seek(SeekFrom::End(0))
            .map_err(|e| format!("无法获取文件大小: {}", e))?;

        if file_size < crate::structures::PcapFileHeader::HEADER_SIZE as u64 {
            return Err("文件太小，不是有效的PCAP文件".to_string());
        }

        // 重置到数据区开始位置
        reader
            .seek(SeekFrom::Start(
                crate::structures::PcapFileHeader::HEADER_SIZE as u64,
            ))
            .map_err(|e| format!("无法定位到数据区: {}", e))?;

        let mut packet_count = 0u64;
        let mut buffer = [0u8; crate::structures::DataPacketHeader::HEADER_SIZE];

        loop {
            match reader.read_exact(&mut buffer) {
                Ok(_) => {
                    // 读取数据包头部
                    let header = crate::structures::DataPacketHeader::from_bytes(&buffer)
                        .map_err(|e| format!("读取数据包头部失败: {}", e))?;

                    // 跳过数据包内容
                    let skip_bytes = header.packet_length as i64;
                    reader
                        .seek(SeekFrom::Current(skip_bytes))
                        .map_err(|e| format!("跳过数据包内容失败: {}", e))?;

                    packet_count += 1;
                }
                Err(_) => break, // 到达文件末尾
            }
        }

        Ok(packet_count)
    }

    fn perform_periodic_cleanup(
        &self,
        cache: &mut HashMap<String, FileInfoCacheItem>,
    ) -> Result<(), String> {
        let mut last_cleanup = self.last_cleanup.lock().map_err(|_| "清理时间锁定失败")?;
        let now = SystemTime::now();

        if now.duration_since(*last_cleanup).unwrap_or(Duration::ZERO) >= self.cleanup_interval {
            self.cleanup_expired_entries(cache)?;
            *last_cleanup = now;
        }

        Ok(())
    }

    fn cleanup_expired_entries(
        &self,
        cache: &mut HashMap<String, FileInfoCacheItem>,
    ) -> Result<(), String> {
        let expired_keys: Vec<String> = cache
            .iter()
            .filter(|(_, item)| item.is_expired(self.cache_expiration))
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            cache.remove(&key);
        }

        Ok(())
    }

    pub fn invalidate_file(&self, file_path: &str) -> Result<(), String> {
        let mut cache = self.cache.lock().map_err(|_| "缓存锁定失败")?;
        cache.remove(file_path);
        Ok(())
    }

    pub fn clear(&self) -> Result<(), String> {
        let mut cache = self.cache.lock().map_err(|_| "缓存锁定失败")?;
        cache.clear();
        Ok(())
    }

    pub fn get_statistics(&self) -> Result<CacheStatistics, String> {
        let cache = self.cache.lock().map_err(|_| "缓存锁定失败")?;

        let expired_entries = cache
            .values()
            .filter(|item| item.is_expired(self.cache_expiration))
            .count();

        let last_cleanup = *self.last_cleanup.lock().map_err(|_| "清理时间锁定失败")?;

        Ok(CacheStatistics {
            total_entries: cache.len(),
            max_entries: self.max_entries,
            expired_entries,
            last_cleanup_time: last_cleanup,
        })
    }
}

impl Default for FileInfoCache {
    fn default() -> Self {
        Self::new(1000)
    }
}

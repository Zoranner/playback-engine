use std::fs;
use std::path::{Path, PathBuf};
use log::{debug, info, warn};
use chrono::DateTime;

use crate::types::{ProjectInfo, ProjectMetadata, PlaybackError, Result};
use crate::pcap_reader::PcapReader;

/// 工程管理器
pub struct ProjectManager {
    current_project: Option<ProjectInfo>,
    pcap_files: Vec<PathBuf>,
}

impl ProjectManager {
    /// 创建新的工程管理器
    pub fn new() -> Self {
        Self {
            current_project: None,
            pcap_files: Vec::new(),
        }
    }
    
    /// 打开工程目录
    pub async fn open_project<P: AsRef<Path>>(&mut self, project_path: P) -> Result<ProjectInfo> {
        let path = project_path.as_ref();
        
        if !path.exists() {
            return Err(PlaybackError::ProjectError(
                format!("工程目录不存在: {:?}", path)
            ));
        }
        
        if !path.is_dir() {
            return Err(PlaybackError::ProjectError(
                format!("指定路径不是目录: {:?}", path)
            ));
        }
        
        info!("正在打开工程目录: {:?}", path);
        
        // 扫描PCAP文件
        let pcap_files = self.scan_pcap_files(path)?;
        
        if pcap_files.is_empty() {
            return Err(PlaybackError::ProjectError(
                "工程目录中未找到PCAP文件".to_string()
            ));
        }
        
        info!("找到 {} 个PCAP文件", pcap_files.len());
        
        // 创建工程信息
        let project_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("未命名工程")
            .to_string();
            
        let mut project_info = ProjectInfo::new(
            project_name,
            path.to_string_lossy().to_string()
        );
        
        // 分析PCAP文件获取工程信息
        self.analyze_pcap_files(&pcap_files, &mut project_info).await?;
        
        // 加载工程元数据（如果存在）
        self.load_project_metadata(path, &mut project_info)?;
        
        // 更新状态
        self.current_project = Some(project_info.clone());
        self.pcap_files = pcap_files;
        
        info!("成功打开工程: {}", project_info.name);
        debug!("工程信息: {:?}", project_info);
        
        Ok(project_info)
    }
    
    /// 扫描目录中的PCAP文件
    fn scan_pcap_files<P: AsRef<Path>>(&self, dir_path: P) -> Result<Vec<PathBuf>> {
        let mut pcap_files = Vec::new();
        let entries = fs::read_dir(dir_path)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.to_str() == Some("pcap") {
                        // 验证文件名格式（可选）
                        if self.is_valid_pcap_filename(&path) {
                            pcap_files.push(path);
                        } else {
                            warn!("PCAP文件名格式不符合规范: {:?}", path);
                        }
                    }
                }
            }
        }
        
        // 按文件名排序（通常包含时间戳）
        pcap_files.sort();
        
        debug!("扫描到PCAP文件: {:?}", pcap_files);
        Ok(pcap_files)
    }
    
    /// 验证PCAP文件名格式
    fn is_valid_pcap_filename(&self, file_path: &Path) -> bool {
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            // 检查是否符合 data_yyMMdd_HHmmss_fffffff.pcap 格式
            if file_name.starts_with("data_") && file_name.ends_with(".pcap") {
                let parts: Vec<&str> = file_name.strip_suffix(".pcap").unwrap().split('_').collect();
                return parts.len() == 4; // data, yyMMdd, HHmmss, fffffff
            }
        }
        false
    }
    
    /// 分析PCAP文件获取工程信息
    async fn analyze_pcap_files(&self, pcap_files: &[PathBuf], project_info: &mut ProjectInfo) -> Result<()> {
        let mut total_duration = 0u64;
        let mut earliest_time: Option<u64> = None;
        let mut latest_time: Option<u64> = None;
        
        for file_path in pcap_files {
            debug!("分析PCAP文件: {:?}", file_path);
            
            match self.analyze_single_pcap_file(file_path).await {
                Ok((duration, start_time, end_time)) => {
                    total_duration += duration;
                    
                    // 更新最早和最晚时间
                    if earliest_time.is_none() || start_time < earliest_time.unwrap() {
                        earliest_time = Some(start_time);
                    }
                    if latest_time.is_none() || end_time > latest_time.unwrap() {
                        latest_time = Some(end_time);
                    }
                    
                    // 添加文件路径到工程信息
                    project_info.pcap_files.push(file_path.to_string_lossy().to_string());
                }
                Err(e) => {
                    warn!("分析PCAP文件失败: {:?}, 错误: {}", file_path, e);
                    // 继续处理其他文件
                }
            }
        }
        
        // 更新工程信息
        project_info.file_count = pcap_files.len();
        project_info.total_duration = total_duration;
        
        // 转换时间戳为ISO格式字符串
        if let Some(earliest) = earliest_time {
            project_info.start_time = self.timestamp_to_iso_string(earliest);
        }
        if let Some(latest) = latest_time {
            project_info.end_time = self.timestamp_to_iso_string(latest);
        }
        
        info!("工程分析完成 - 文件数: {}, 总时长: {:.2}秒", 
              project_info.file_count, 
              total_duration as f64 / 1_000_000_000.0);
              
        Ok(())
    }
    
    /// 分析单个PCAP文件
    async fn analyze_single_pcap_file(&self, file_path: &Path) -> Result<(u64, u64, u64)> {
        let mut reader = PcapReader::new(file_path)?;
        
        // 获取文件时长
        let duration = reader.get_total_duration()?;
        
        // 读取第一个数据包获取开始时间
        reader.reset()?;
        let start_time = if let Some(first_packet) = reader.read_next_packet()? {
            first_packet.get_timestamp_ns()
        } else {
            0
        };
        
        // 计算结束时间
        let end_time = start_time + duration;
        
        debug!("文件 {:?} - 时长: {}ns, 开始: {}ns, 结束: {}ns", 
               file_path, duration, start_time, end_time);
        
        Ok((duration, start_time, end_time))
    }
    
    /// 将纳秒时间戳转换为ISO格式字符串
    fn timestamp_to_iso_string(&self, timestamp_ns: u64) -> String {
        let timestamp_secs = (timestamp_ns / 1_000_000_000) as i64;
        let nanosecs = (timestamp_ns % 1_000_000_000) as u32;
        
        if let Some(datetime) = DateTime::from_timestamp(timestamp_secs, nanosecs) {
            datetime.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
        } else {
            "1970-01-01T00:00:00.000Z".to_string()
        }
    }
    
    /// 加载工程元数据
    fn load_project_metadata<P: AsRef<Path>>(&self, project_path: P, project_info: &mut ProjectInfo) -> Result<()> {
        let metadata_file = project_path.as_ref().join("project.json");
        
        if metadata_file.exists() {
            debug!("加载工程元数据文件: {:?}", metadata_file);
            
            match fs::read_to_string(&metadata_file) {
                Ok(content) => {
                    match serde_json::from_str::<ProjectMetadata>(&content) {
                        Ok(metadata) => {
                            project_info.metadata = metadata;
                            info!("成功加载工程元数据");
                        }
                        Err(e) => {
                            warn!("解析工程元数据失败: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("读取工程元数据文件失败: {}", e);
                }
            }
        } else {
            debug!("工程元数据文件不存在，使用默认配置");
        }
        
        Ok(())
    }
    
    /// 保存工程元数据
    pub fn save_project_metadata(&self) -> Result<()> {
        if let Some(project) = &self.current_project {
            let metadata_file = Path::new(&project.path).join("project.json");
            
            let content = serde_json::to_string_pretty(&project.metadata)?;
            fs::write(&metadata_file, content)?;
            
            info!("工程元数据已保存到: {:?}", metadata_file);
        }
        
        Ok(())
    }
    
    /// 获取当前工程信息
    pub fn get_current_project(&self) -> Option<&ProjectInfo> {
        self.current_project.as_ref()
    }
    
    /// 获取PCAP文件列表
    pub fn get_pcap_files(&self) -> &[PathBuf] {
        &self.pcap_files
    }
    
    /// 关闭当前工程
    pub fn close_project(&mut self) {
        if let Some(project) = &self.current_project {
            info!("关闭工程: {}", project.name);
        }
        
        self.current_project = None;
        self.pcap_files.clear();
    }
    
    /// 验证工程目录
    pub fn validate_project_directory<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(PlaybackError::ProjectError(
                format!("目录不存在: {:?}", path)
            ));
        }
        
        if !path.is_dir() {
            return Err(PlaybackError::ProjectError(
                format!("路径不是目录: {:?}", path)
            ));
        }
        
        // 检查是否有PCAP文件
        let entries = fs::read_dir(path)?;
        let has_pcap = entries
            .filter_map(|entry| entry.ok())
            .any(|entry| {
                entry.path().extension()
                    .and_then(|ext| ext.to_str())
                    == Some("pcap")
            });
            
        if !has_pcap {
            return Err(PlaybackError::ProjectError(
                "目录中未找到PCAP文件".to_string()
            ));
        }
        
        Ok(())
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
} 
use std::path::Path;
use log::info;

use crate::types::common::{PlaybackError, Result, ProjectInfo};
use crate::project::structure::ProjectStructure;

/// 项目加载器
pub struct ProjectLoader {
    current_project: Option<ProjectInfo>,
    project_structure: Option<ProjectStructure>,
}

impl ProjectLoader {
    /// 创建新的项目加载器
    pub fn new() -> Self {
        Self {
            current_project: None,
            project_structure: None,
        }
    }

    /// 打开工程目录
    pub async fn open_project<P: AsRef<Path>>(
        &mut self,
        project_path: P,
    ) -> Result<ProjectInfo> {
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

        // 1. 构建工程结构
        let structure = ProjectStructure::from_path(path)?;
        
        // 2. 创建工程信息
        let project_info = structure.to_project_info()?;
        
        // 3. 保存状态
        self.project_structure = Some(structure);
        self.current_project = Some(project_info.clone());

        info!("工程打开成功: {}", project_info.name);
        Ok(project_info)
    }

    /// 获取当前工程信息
    pub fn current_project(&self) -> Option<ProjectInfo> {
        self.current_project.clone()
    }

    /// 关闭当前工程
    pub fn close_project(&mut self) {
        self.current_project = None;
        self.project_structure = None;
        info!("工程已关闭");
    }
}
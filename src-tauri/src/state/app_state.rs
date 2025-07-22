use crate::types::common::ProjectInfo;
use crate::playback::engine::PlaybackEngine;
use std::sync::Arc;
use crate::state::playback_state::PlaybackState;

/// 应用全局状态
#[derive(Debug)]
pub struct AppState {
    current_project: Option<ProjectInfo>,
    pub playback_engine: PlaybackEngine,
}

impl AppState {
    pub fn new() -> Self {
        let playback_state = Arc::new(tokio::sync::Mutex::new(
            PlaybackState::new()
        ));

        Self {
            current_project: None,
            playback_engine: PlaybackEngine::new(playback_state),
        }
    }

    pub fn current_project(&self) -> Option<ProjectInfo> {
        self.current_project.clone()
    }

    pub fn set_current_project(
        &mut self,
        project: Option<ProjectInfo>
    ) {
        self.current_project = project;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

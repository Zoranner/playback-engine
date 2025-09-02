use serde::{Deserialize, Serialize};

/// 回放状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlaybackStatus {
    Stopped,
    Playing,
    Paused,
    Completed,
}

/// 回放状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackState {
    pub current_dataset: Option<String>,
    pub current_timestamp: u64,
    pub total_duration: u64,
    pub playback_speed: f64,
    pub status: PlaybackStatus,
    pub current_packet_index: u64,
    pub total_packets: u64,
}

impl PlaybackState {
    pub fn new() -> Self {
        Self {
            current_dataset: None,
            current_timestamp: 0,
            total_duration: 0,
            playback_speed: 1.0,
            status: PlaybackStatus::Stopped,
            current_packet_index: 0,
            total_packets: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.status, PlaybackStatus::Playing)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self.status, PlaybackStatus::Paused)
    }
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self::new()
    }
}

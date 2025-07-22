//! 时间轴控制

#[derive(Debug)]
pub struct TimelineController {
    start_time: u64,
    end_time: u64,
    current_time: u64,
    playback_speed: f64,
}

impl TimelineController {
    pub fn new(start: u64, end: u64) -> Self {
        Self {
            start_time: start,
            end_time: end,
            current_time: start,
            playback_speed: 1.0,
        }
    }

    pub fn set_current_time(&mut self, time: u64) {
        self.current_time = time.clamp(self.start_time, self.end_time);
    }

    pub fn get_current_time(&self) -> u64 {
        self.current_time
    }

    pub fn get_progress(&self) -> f64 {
        if self.end_time > self.start_time {
            (self.current_time - self.start_time) as f64 / (self.end_time - self.start_time) as f64
        } else {
            0.0
        }
    }

    pub fn set_playback_speed(&mut self, speed: f64) {
        self.playback_speed = speed.max(0.1).min(10.0); // 限制速度范围 0.1x - 10x
    }

    pub fn get_playback_speed(&self) -> f64 {
        self.playback_speed
    }

    pub fn advance_time(&mut self, delta_ms: u64) -> bool {
        let advance_amount = (delta_ms as f64 * self.playback_speed) as u64;
        let new_time = self.current_time + advance_amount;
        
        if new_time >= self.end_time {
            self.current_time = self.end_time;
            true // 表示播放结束
        } else {
            self.current_time = new_time;
            false
        }
    }

    pub fn get_duration(&self) -> u64 {
        self.end_time - self.start_time
    }

    pub fn is_at_end(&self) -> bool {
        self.current_time >= self.end_time
    }

    pub fn reset(&mut self) {
        self.current_time = self.start_time;
    }
}
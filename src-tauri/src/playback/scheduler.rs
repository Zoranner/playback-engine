//! 事件调度器

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub struct ScheduledEvent {
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub dataset: String,
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for ScheduledEvent {}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.timestamp.partial_cmp(&self.timestamp)
    }
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timestamp.cmp(&self.timestamp)
    }
}

#[derive(Debug)]
pub struct EventScheduler {
    events: BinaryHeap<ScheduledEvent>,
}

impl EventScheduler {
    pub fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
        }
    }

    pub fn add_event(&mut self, event: ScheduledEvent) {
        self.events.push(event);
    }

    pub fn get_next_event(&mut self, current_time: u64) -> Option<ScheduledEvent> {
        if let Some(event) = self.events.peek() {
            if event.timestamp <= current_time {
                return self.events.pop();
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

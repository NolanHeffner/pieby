
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct TimeManagement {
    start: Instant,
    duration: Duration,
    end: Option<Instant>,
    node_limit: u64,
}

impl TimeManagement {
    pub fn new(duration: Duration, node_limit: u64) -> Self {
        let current_time = Instant::now();
        Self {
            start: current_time,
            duration,
            end: Some(current_time + duration),
            node_limit,
        }
    }

    pub fn set_node_limit(&mut self, node_limit: u64) {
        self.node_limit = node_limit;
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn time_exceeded(&self) -> bool {
        if let Some(end) = self.end {
            return Instant::now() > end;
        }
        false
    }
}
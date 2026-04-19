use crate::log_entry::LogEntry;
use crate::rate_config::RateConfig;
use std::collections::VecDeque;

/// Computes the rate of log entries over a sliding window.
pub struct RateFilter {
    config: RateConfig,
    window: VecDeque<i64>,
}

impl RateFilter {
    pub fn new(config: RateConfig) -> Self {
        Self {
            config,
            window: VecDeque::new(),
        }
    }

    /// Returns true if the entry should be kept (rate within limit).
    pub fn accept(&mut self, entry: &LogEntry) -> bool {
        let ts = entry.timestamp_ms;
        let window_start = ts - self.config.window_ms;

        // Evict old entries
        while let Some(&front) = self.window.front() {
            if front < window_start {
                self.window.pop_front();
            } else {
                break;
            }
        }

        if self.window.len() < self.config.max_count {
            self.window.push_back(ts);
            true
        } else {
            false
        }
    }

    /// Apply rate filter to a slice of entries.
    pub fn apply(&mut self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().filter(|e| self.accept(e)).collect()
    }

    pub fn current_rate(&self) -> usize {
        self.window.len()
    }
}

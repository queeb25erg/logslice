use std::collections::VecDeque;
use crate::log_entry::LogEntry;
use crate::window_config::WindowConfig;

/// Collects log entries into fixed-size or time-based sliding windows.
pub struct Window {
    config: WindowConfig,
    buffer: VecDeque<LogEntry>,
}

impl Window {
    pub fn new(config: WindowConfig) -> Self {
        Self {
            config,
            buffer: VecDeque::new(),
        }
    }

    /// Push an entry into the window. Returns a completed window if full.
    pub fn push(&mut self, entry: LogEntry) -> Option<Vec<LogEntry>> {
        self.buffer.push_back(entry);
        if self.buffer.len() >= self.config.size {
            let window: Vec<LogEntry> = self.buffer.drain(..).collect();
            return Some(window);
        }
        None
    }

    /// Slide the window by `step` entries and return the current window view.
    pub fn slide(&mut self, entry: LogEntry) -> Option<Vec<LogEntry>> {
        self.buffer.push_back(entry);
        if self.buffer.len() > self.config.size {
            for _ in 0..self.config.step {
                self.buffer.pop_front();
            }
        }
        if self.buffer.len() == self.config.size {
            return Some(self.buffer.iter().cloned().collect());
        }
        None
    }

    /// Flush remaining buffered entries regardless of window size.
    pub fn flush(&mut self) -> Option<Vec<LogEntry>> {
        if self.buffer.is_empty() {
            return None;
        }
        Some(self.buffer.drain(..).collect())
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

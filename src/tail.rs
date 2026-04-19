use crate::log_entry::LogEntry;

/// Configuration for tailing the last N log entries.
#[derive(Debug, Clone)]
pub struct TailConfig {
    pub count: usize,
}

impl TailConfig {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

/// Returns the last `config.count` entries from the provided slice.
pub fn tail_entries<'a>(entries: &'a [LogEntry], config: &TailConfig) -> &'a [LogEntry] {
    if config.count == 0 || entries.is_empty() {
        return &[];
    }
    let start = entries.len().saturating_sub(config.count);
    &entries[start..]
}

/// Returns the last `config.count` entries, consuming an iterator.
pub fn tail_from_iter(iter: impl Iterator<Item = LogEntry>, config: &TailConfig) -> Vec<LogEntry> {
    if config.count == 0 {
        return vec![];
    }
    let mut ring: std::collections::VecDeque<LogEntry> = std::collections::VecDeque::with_capacity(config.count + 1);
    for entry in iter {
        if ring.len() == config.count {
            ring.pop_front();
        }
        ring.push_back(entry);
    }
    ring.into_iter().collect()
}

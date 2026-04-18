use crate::log_entry::LogEntry;
use crate::stats::LogStats;

pub struct StatsCollector {
    stats: LogStats,
    enabled: bool,
}

impl StatsCollector {
    pub fn new(enabled: bool) -> Self {
        Self {
            stats: LogStats::new(),
            enabled,
        }
    }

    pub fn observe(&mut self, entry: &LogEntry) {
        if self.enabled {
            self.stats.record(entry);
        }
    }

    pub fn observe_all(&mut self, entries: &[LogEntry]) {
        for e in entries {
            self.observe(e);
        }
    }

    pub fn print_summary(&self) {
        if self.enabled {
            eprintln!("\n--- Log Statistics ---");
            eprintln!("{}", self.stats.summary());
        }
    }

    pub fn stats(&self) -> &LogStats {
        &self.stats
    }
}

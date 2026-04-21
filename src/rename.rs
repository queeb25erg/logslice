use crate::log_entry::LogEntry;
use crate::rename_config::RenameConfig;

/// Renames fields in a log entry according to the provided configuration.
pub struct Renamer {
    config: RenameConfig,
}

impl Renamer {
    pub fn new(config: RenameConfig) -> Self {
        Self { config }
    }

    /// Apply field renames to a single log entry.
    /// Returns a new LogEntry with renamed fields.
    pub fn apply(&self, mut entry: LogEntry) -> LogEntry {
        for (from, to) in &self.config.mappings {
            if let Some(value) = entry.fields.remove(from.as_str()) {
                entry.fields.insert(to.clone(), value);
            }
        }
        entry
    }

    /// Apply renames to a collection of log entries.
    pub fn apply_all(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().map(|e| self.apply(e)).collect()
    }
}

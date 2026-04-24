use crate::log_entry::LogEntry;
use crate::coalesce_config::CoalesceConfig;

/// Coalesces multiple fields into a single target field,
/// using the first non-null/non-empty value found.
pub struct Coalescer {
    config: CoalesceConfig,
}

impl Coalescer {
    pub fn new(config: CoalesceConfig) -> Self {
        Self { config }
    }

    /// Apply coalescing to a single log entry.
    /// Sets `target` to the first non-empty value among `sources`.
    pub fn apply(&self, entry: &mut LogEntry) -> bool {
        for source in &self.config.sources {
            if let Some(val) = entry.fields.get(source) {
                let val = val.clone();
                if !val.is_null() && val.as_str().map(|s| !s.is_empty()).unwrap_or(true) {
                    entry.fields.insert(self.config.target.clone(), val);
                    if self.config.remove_sources {
                        for s in &self.config.sources {
                            if s != &self.config.target {
                                entry.fields.remove(s);
                            }
                        }
                    }
                    return true;
                }
            }
        }
        // If a fallback is set, apply it
        if let Some(ref fallback) = self.config.fallback {
            entry.fields.insert(
                self.config.target.clone(),
                serde_json::Value::String(fallback.clone()),
            );
            return true;
        }
        false
    }

    /// Apply coalescing to a batch of log entries.
    pub fn apply_all(&self, entries: &mut Vec<LogEntry>) {
        for entry in entries.iter_mut() {
            self.apply(entry);
        }
    }
}

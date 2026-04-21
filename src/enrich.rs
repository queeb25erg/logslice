use crate::enrich_config::EnrichConfig;
use crate::log_entry::LogEntry;
use std::collections::HashMap;

/// Enriches log entries by adding or overwriting fields with static or derived values.
pub struct Enrich {
    config: EnrichConfig,
}

impl Enrich {
    pub fn new(config: EnrichConfig) -> Self {
        Self { config }
    }

    /// Apply enrichment to a single log entry, returning a modified copy.
    pub fn apply(&self, mut entry: LogEntry) -> LogEntry {
        for (key, value) in &self.config.static_fields {
            entry.fields.insert(key.clone(), value.clone());
        }

        if let Some(ref copy_rules) = self.config.copy_fields {
            for (src, dst) in copy_rules {
                if let Some(val) = entry.fields.get(src).cloned() {
                    entry.fields.insert(dst.clone(), val);
                }
            }
        }

        if let Some(ref rename_rules) = self.config.rename_fields {
            for (old_key, new_key) in rename_rules {
                if let Some(val) = entry.fields.remove(old_key) {
                    entry.fields.insert(new_key.clone(), val);
                }
            }
        }

        entry
    }

    /// Apply enrichment to a batch of log entries.
    pub fn apply_all(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().map(|e| self.apply(e)).collect()
    }
}

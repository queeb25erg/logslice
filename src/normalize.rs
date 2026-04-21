use crate::log_entry::LogEntry;
use crate::normalize_config::NormalizeConfig;
use std::collections::HashMap;

/// Normalizes fields in a log entry according to the provided config.
pub struct Normalizer {
    config: NormalizeConfig,
}

impl Normalizer {
    pub fn new(config: NormalizeConfig) -> Self {
        Self { config }
    }

    /// Apply normalization to a single log entry, returning a new normalized entry.
    pub fn normalize(&self, entry: LogEntry) -> LogEntry {
        let fields = self.normalize_fields(entry.fields);
        LogEntry { fields, ..entry }
    }

    fn normalize_fields(&self, fields: HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        for (key, value) in fields {
            let should_process = self.config.all_fields || self.config.fields.contains(&key);

            let new_key = if should_process && self.config.lowercase_keys {
                key.to_lowercase()
            } else {
                key
            };

            let new_value = if should_process && self.config.trim_values {
                trim_value(value)
            } else {
                value
            };

            result.insert(new_key, new_value);
        }
        result
    }

    /// Apply normalization to a batch of log entries.
    pub fn normalize_all(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().map(|e| self.normalize(e)).collect()
    }
}

fn trim_value(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::String(s) => serde_json::Value::String(s.trim().to_string()),
        other => other,
    }
}

#[cfg(test)]
#[path = "normalize_tests.rs"]
mod tests;

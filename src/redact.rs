use crate::log_entry::LogEntry;
use crate::redact_config::RedactConfig;

/// Redacts specified fields in a log entry by replacing their values with a mask string.
pub struct Redactor {
    config: RedactConfig,
}

impl Redactor {
    pub fn new(config: RedactConfig) -> Self {
        Self { config }
    }

    pub fn redact(&self, entry: &mut LogEntry) {
        for field in &self.config.fields {
            if entry.fields.contains_key(field.as_str()) {
                entry.fields.insert(field.clone(), self.config.mask.clone());
            }
        }
    }

    pub fn redact_owned(&self, mut entry: LogEntry) -> LogEntry {
        self.redact(&mut entry);
        entry
    }

    pub fn redact_all(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().map(|e| self.redact_owned(e)).collect()
    }
}

#[cfg(test)]
#[path = "redact_tests.rs"]
mod tests;

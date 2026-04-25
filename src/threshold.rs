use crate::log_entry::LogEntry;
use crate::threshold_config::ThresholdConfig;

/// Filters log entries based on a numeric field threshold.
pub struct Threshold {
    config: ThresholdConfig,
}

impl Threshold {
    pub fn new(config: ThresholdConfig) -> Self {
        Self { config }
    }

    /// Returns true if the entry's field value meets the threshold condition.
    pub fn matches(&self, entry: &LogEntry) -> bool {
        match entry.fields.get(&self.config.field) {
            Some(val) => {
                if let Ok(n) = val.parse::<f64>() {
                    self.config.matches(n)
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Filters a slice of entries, returning only those that meet the threshold.
    pub fn apply(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().filter(|e| self.matches(e)).collect()
    }

    /// Annotates matching entries with a threshold label field.
    pub fn annotate(&self, mut entries: Vec<LogEntry>) -> Vec<LogEntry> {
        for entry in &mut entries {
            if self.matches(entry) {
                entry
                    .fields
                    .insert("threshold".to_string(), self.config.effective_label().to_string());
            }
        }
        entries
    }
}

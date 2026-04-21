use crate::mask_config::MaskConfig;
use crate::log_entry::LogEntry;

/// Applies field masking to log entries, replacing field values with a
/// fixed mask string (e.g. `***`) based on a list of field names.
pub struct Masker {
    config: MaskConfig,
}

impl Masker {
    pub fn new(config: MaskConfig) -> Self {
        Self { config }
    }

    /// Apply masking to a single log entry, returning a modified copy.
    pub fn apply(&self, mut entry: LogEntry) -> LogEntry {
        for field in &self.config.fields {
            if entry.fields.contains_key(field.as_str()) {
                entry
                    .fields
                    .insert(field.clone(), self.config.mask.clone());
            }
        }
        entry
    }

    /// Apply masking to a collection of log entries.
    pub fn apply_all(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().map(|e| self.apply(e)).collect()
    }

    /// Returns true if the masker has any fields configured.
    pub fn is_active(&self) -> bool {
        !self.config.fields.is_empty()
    }
}

use crate::log_entry::LogEntry;
use crate::annotate_config::AnnotateConfig;

/// Annotates log entries by adding a new field derived from existing fields or static values.
pub struct Annotator {
    config: AnnotateConfig,
}

impl Annotator {
    pub fn new(config: AnnotateConfig) -> Self {
        Self { config }
    }

    /// Annotate a single log entry, returning a modified copy.
    pub fn annotate(&self, mut entry: LogEntry) -> LogEntry {
        let value = match &self.config.source_field {
            Some(src) => {
                if let Some(existing) = entry.fields.get(src) {
                    let val = existing.clone();
                    self.apply_prefix_suffix(val)
                } else {
                    self.config.default_value.clone().unwrap_or_default()
                }
            }
            None => self.config.default_value.clone().unwrap_or_default(),
        };

        entry.fields.insert(self.config.target_field.clone(), value);
        entry
    }

    /// Annotate a batch of log entries.
    pub fn annotate_all(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().map(|e| self.annotate(e)).collect()
    }

    fn apply_prefix_suffix(&self, value: String) -> String {
        let prefix = self.config.prefix.as_deref().unwrap_or("");
        let suffix = self.config.suffix.as_deref().unwrap_or("");
        format!("{}{}{}", prefix, value, suffix)
    }
}

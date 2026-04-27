use crate::log_entry::LogEntry;
use crate::classify_config::ClassifyConfig;

/// Classifies log entries by assigning a category label based on field pattern matching.
pub struct Classifier {
    config: ClassifyConfig,
}

impl Classifier {
    pub fn new(config: ClassifyConfig) -> Self {
        Self { config }
    }

    /// Applies classification rules to a log entry, returning a new entry
    /// with the category field added (or updated).
    pub fn classify(&self, mut entry: LogEntry) -> LogEntry {
        for rule in &self.config.rules {
            if let Some(value) = entry.fields.get(&rule.field) {
                if rule.pattern.is_match(value) {
                    entry
                        .fields
                        .insert(self.config.output_field.clone(), rule.category.clone());
                    return entry;
                }
            }
        }
        if let Some(ref default) = self.config.default_category {
            entry
                .fields
                .insert(self.config.output_field.clone(), default.clone());
        }
        entry
    }

    /// Classifies a batch of log entries.
    pub fn classify_all(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().map(|e| self.classify(e)).collect()
    }
}

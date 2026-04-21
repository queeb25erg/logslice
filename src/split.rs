use crate::log_entry::LogEntry;
use crate::split_config::SplitConfig;
use std::collections::HashMap;

/// Splits a stream of log entries into named buckets based on a field value.
pub struct Splitter {
    config: SplitConfig,
}

impl Splitter {
    pub fn new(config: SplitConfig) -> Self {
        Self { config }
    }

    /// Groups entries by the value of the configured field.
    /// Entries missing the field go into the "_unknown" bucket.
    pub fn split(&self, entries: Vec<LogEntry>) -> HashMap<String, Vec<LogEntry>> {
        let mut buckets: HashMap<String, Vec<LogEntry>> = HashMap::new();

        for entry in entries {
            let key = entry
                .fields
                .get(&self.config.field)
                .cloned()
                .unwrap_or_else(|| self.config.fallback_key.clone());

            let key = if self.config.max_buckets > 0
                && buckets.len() >= self.config.max_buckets
                && !buckets.contains_key(&key)
            {
                self.config.overflow_key.clone()
            } else {
                key
            };

            buckets.entry(key).or_default().push(entry);
        }

        buckets
    }

    /// Returns sorted bucket names for deterministic output.
    pub fn bucket_names(buckets: &HashMap<String, Vec<LogEntry>>) -> Vec<String> {
        let mut names: Vec<String> = buckets.keys().cloned().collect();
        names.sort();
        names
    }
}

use crate::log_entry::LogEntry;
use crate::rollup_config::RollupConfig;
use std::collections::HashMap;

/// Rolls up multiple log entries into summary entries grouped by a key field.
pub struct Rollup {
    config: RollupConfig,
}

#[derive(Debug, Clone)]
pub struct RollupBucket {
    pub key: String,
    pub count: usize,
    pub entries: Vec<LogEntry>,
}

impl Rollup {
    pub fn new(config: RollupConfig) -> Self {
        Self { config }
    }

    /// Groups entries by the configured key field and returns rollup buckets.
    pub fn apply(&self, entries: &[LogEntry]) -> Vec<RollupBucket> {
        let mut map: HashMap<String, Vec<LogEntry>> = HashMap::new();

        for entry in entries {
            let key = entry
                .fields
                .get(&self.config.key_field)
                .cloned()
                .unwrap_or_else(|| self.config.default_key.clone());
            map.entry(key).or_default().push(entry.clone());
        }

        let mut buckets: Vec<RollupBucket> = map
            .into_iter()
            .map(|(key, entries)| {
                let count = entries.len();
                RollupBucket { key, count, entries }
            })
            .collect();

        buckets.sort_by(|a, b| b.count.cmp(&a.count));

        if let Some(limit) = self.config.top_n {
            buckets.truncate(limit);
        }

        buckets
    }

    /// Converts rollup buckets into summary LogEntry records.
    pub fn to_summary_entries(&self, buckets: &[RollupBucket]) -> Vec<LogEntry> {
        buckets
            .iter()
            .map(|b| {
                let mut fields = HashMap::new();
                fields.insert(self.config.key_field.clone(), b.key.clone());
                fields.insert("count".to_string(), b.count.to_string());
                if self.config.include_first && !b.entries.is_empty() {
                    if let Some(ts) = &b.entries[0].timestamp {
                        fields.insert("first_timestamp".to_string(), ts.clone());
                    }
                }
                LogEntry {
                    raw: format!("rollup: {}={} count={}", self.config.key_field, b.key, b.count),
                    timestamp: None,
                    fields,
                }
            })
            .collect()
    }
}

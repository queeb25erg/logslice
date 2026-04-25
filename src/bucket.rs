use std::collections::HashMap;
use crate::bucket_config::BucketConfig;
use crate::log_entry::LogEntry;

/// Groups log entries into time-based or value-based buckets.
pub struct Bucket {
    config: BucketConfig,
    buckets: HashMap<String, Vec<LogEntry>>,
}

impl Bucket {
    pub fn new(config: BucketConfig) -> Self {
        Self {
            config,
            buckets: HashMap::new(),
        }
    }

    /// Insert a log entry into the appropriate bucket.
    pub fn insert(&mut self, entry: LogEntry) {
        let key = self.bucket_key(&entry);
        self.buckets.entry(key).or_default().push(entry);
    }

    /// Compute the bucket key for a given log entry.
    fn bucket_key(&self, entry: &LogEntry) -> String {
        if let Some(ref field) = self.config.field {
            entry
                .fields
                .get(field)
                .cloned()
                .unwrap_or_else(|| self.config.default_key.clone())
        } else if let Some(interval_secs) = self.config.interval_secs {
            // Time-based bucketing: floor timestamp to nearest interval
            let ts = entry.timestamp.timestamp();
            let bucket_ts = (ts / interval_secs as i64) * interval_secs as i64;
            bucket_ts.to_string()
        } else {
            self.config.default_key.clone()
        }
    }

    /// Returns all buckets as a sorted list of (key, entries) pairs.
    pub fn into_sorted(self) -> Vec<(String, Vec<LogEntry>)> {
        let mut pairs: Vec<(String, Vec<LogEntry>)> = self.buckets.into_iter().collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        pairs
    }

    /// Returns the number of distinct buckets.
    pub fn bucket_count(&self) -> usize {
        self.buckets.len()
    }

    /// Returns the total number of entries across all buckets.
    pub fn total_entries(&self) -> usize {
        self.buckets.values().map(|v| v.len()).sum()
    }
}

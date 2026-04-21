/// Configuration for the log entry merger.
#[derive(Debug, Clone)]
pub struct MergeConfig {
    /// The field name used as the sort key when merging streams.
    pub timestamp_field: String,
    /// Whether to deduplicate entries with identical timestamps and content.
    pub dedup: bool,
}

impl MergeConfig {
    pub fn new(timestamp_field: impl Into<String>) -> Self {
        Self {
            timestamp_field: timestamp_field.into(),
            dedup: false,
        }
    }

    pub fn with_dedup(mut self, dedup: bool) -> Self {
        self.dedup = dedup;
        self
    }
}

impl Default for MergeConfig {
    fn default() -> Self {
        Self::new("timestamp")
    }
}

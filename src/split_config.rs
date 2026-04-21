/// Configuration for the log entry splitter.
#[derive(Debug, Clone, PartialEq)]
pub struct SplitConfig {
    /// The field whose value determines the bucket.
    pub field: String,
    /// Key used when an entry does not contain the field.
    pub fallback_key: String,
    /// Maximum number of distinct buckets allowed (0 = unlimited).
    pub max_buckets: usize,
    /// Key used for entries that would exceed max_buckets.
    pub overflow_key: String,
}

impl SplitConfig {
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            fallback_key: "_unknown".to_string(),
            max_buckets: 0,
            overflow_key: "_overflow".to_string(),
        }
    }

    pub fn with_fallback_key(mut self, key: impl Into<String>) -> Self {
        self.fallback_key = key.into();
        self
    }

    pub fn with_max_buckets(mut self, max: usize) -> Self {
        self.max_buckets = max;
        self
    }

    pub fn with_overflow_key(mut self, key: impl Into<String>) -> Self {
        self.overflow_key = key.into();
        self
    }
}

impl Default for SplitConfig {
    fn default() -> Self {
        Self::new("level")
    }
}

/// Configuration for the Bucket module.
#[derive(Debug, Clone)]
pub struct BucketConfig {
    /// Field name to bucket by (value-based bucketing).
    /// If None, falls back to time-based bucketing.
    pub field: Option<String>,

    /// Interval in seconds for time-based bucketing.
    /// Only used when `field` is None.
    pub interval_secs: Option<u64>,

    /// Default bucket key for entries with missing field values.
    pub default_key: String,
}

impl BucketConfig {
    /// Create a field-based bucket config.
    pub fn by_field(field: impl Into<String>) -> Self {
        Self {
            field: Some(field.into()),
            interval_secs: None,
            default_key: "__unknown__".to_string(),
        }
    }

    /// Create a time-interval bucket config.
    pub fn by_interval(interval_secs: u64) -> Self {
        Self {
            field: None,
            interval_secs: Some(interval_secs),
            default_key: "__unknown__".to_string(),
        }
    }

    /// Override the default key used for missing values.
    pub fn with_default_key(mut self, key: impl Into<String>) -> Self {
        self.default_key = key.into();
        self
    }
}

impl Default for BucketConfig {
    fn default() -> Self {
        Self {
            field: None,
            interval_secs: Some(60),
            default_key: "__unknown__".to_string(),
        }
    }
}

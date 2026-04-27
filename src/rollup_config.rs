/// Configuration for the Rollup module.
#[derive(Debug, Clone)]
pub struct RollupConfig {
    /// The field name to group entries by.
    pub key_field: String,
    /// Value to use when the key field is missing.
    pub default_key: String,
    /// If set, only keep the top N groups by count.
    pub top_n: Option<usize>,
    /// Whether to include the first entry's timestamp in the summary.
    pub include_first: bool,
}

impl RollupConfig {
    pub fn new(key_field: impl Into<String>) -> Self {
        Self {
            key_field: key_field.into(),
            default_key: "(unknown)".to_string(),
            top_n: None,
            include_first: false,
        }
    }

    pub fn with_default_key(mut self, key: impl Into<String>) -> Self {
        self.default_key = key.into();
        self
    }

    pub fn with_top_n(mut self, n: usize) -> Self {
        self.top_n = Some(n);
        self
    }

    pub fn with_include_first(mut self, include: bool) -> Self {
        self.include_first = include;
        self
    }
}

impl Default for RollupConfig {
    fn default() -> Self {
        Self::new("level")
    }
}

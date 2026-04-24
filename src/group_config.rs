/// Configuration for the log entry grouping feature.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupConfig {
    /// Fields to group by (in order).
    pub fields: Vec<String>,
    /// Separator used when joining multiple field values into a key.
    pub separator: String,
    /// Placeholder used when a field is missing from an entry.
    pub missing_value: String,
}

impl GroupConfig {
    pub fn new(fields: Vec<String>) -> Self {
        Self {
            fields,
            separator: "|".to_string(),
            missing_value: "<none>".to_string(),
        }
    }

    pub fn with_separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    pub fn with_missing_value(mut self, val: impl Into<String>) -> Self {
        self.missing_value = val.into();
        self
    }
}

impl Default for GroupConfig {
    fn default() -> Self {
        Self::new(vec![])
    }
}

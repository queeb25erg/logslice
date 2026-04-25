/// Configuration for the correlate module.
#[derive(Debug, Clone)]
pub struct CorrelateConfig {
    /// The log entry field to correlate by (e.g. "request_id", "trace_id").
    pub field: String,
    /// Minimum number of entries required to form a group.
    pub min_group_size: usize,
    /// Whether to include entries that lack the correlation field.
    pub include_unkeyed: bool,
    /// Whether to sort entries within each group by timestamp.
    pub sort_within_group: bool,
    /// Whether to annotate each entry with its correlation key.
    pub annotate_key: bool,
    /// Field name used when annotating entries with the correlation key.
    pub annotation_field: String,
}

impl CorrelateConfig {
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            min_group_size: 1,
            include_unkeyed: false,
            sort_within_group: true,
            annotate_key: false,
            annotation_field: String::from("_correlation_key"),
        }
    }

    pub fn min_group_size(mut self, size: usize) -> Self {
        self.min_group_size = size;
        self
    }

    pub fn include_unkeyed(mut self, val: bool) -> Self {
        self.include_unkeyed = val;
        self
    }

    pub fn sort_within_group(mut self, val: bool) -> Self {
        self.sort_within_group = val;
        self
    }

    pub fn annotate_key(mut self, val: bool) -> Self {
        self.annotate_key = val;
        self
    }

    pub fn annotation_field(mut self, field: impl Into<String>) -> Self {
        self.annotation_field = field.into();
        self
    }
}

impl Default for CorrelateConfig {
    fn default() -> Self {
        Self::new("request_id")
    }
}

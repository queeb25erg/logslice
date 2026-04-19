use crate::truncate::TruncateConfig;

impl TruncateConfig {
    pub fn new(max_field_len: usize) -> Self {
        Self {
            max_field_len,
            fields: None,
            suffix: "...".to_string(),
        }
    }

    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }

    pub fn applies_to(&self, field: &str) -> bool {
        self.fields
            .as_ref()
            .map(|f| f.iter().any(|k| k == field))
            .unwrap_or(true)
    }
}

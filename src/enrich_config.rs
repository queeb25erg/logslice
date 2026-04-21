use std::collections::HashMap;

/// Configuration for the log entry enrichment stage.
#[derive(Debug, Clone, PartialEq)]
pub struct EnrichConfig {
    /// Static key-value pairs to inject into every log entry.
    pub static_fields: HashMap<String, String>,
    /// Copy the value of one field into another field (src -> dst).
    pub copy_fields: Option<HashMap<String, String>>,
    /// Rename fields (old_key -> new_key).
    pub rename_fields: Option<HashMap<String, String>>,
}

impl EnrichConfig {
    pub fn new() -> Self {
        Self {
            static_fields: HashMap::new(),
            copy_fields: None,
            rename_fields: None,
        }
    }

    pub fn with_static_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.static_fields.insert(key.into(), value.into());
        self
    }

    pub fn with_copy(mut self, src: impl Into<String>, dst: impl Into<String>) -> Self {
        self.copy_fields
            .get_or_insert_with(HashMap::new)
            .insert(src.into(), dst.into());
        self
    }

    pub fn with_rename(mut self, old_key: impl Into<String>, new_key: impl Into<String>) -> Self {
        self.rename_fields
            .get_or_insert_with(HashMap::new)
            .insert(old_key.into(), new_key.into());
        self
    }
}

impl Default for EnrichConfig {
    fn default() -> Self {
        Self::new()
    }
}

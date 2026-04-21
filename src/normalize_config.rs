/// Configuration for the normalize stage
#[derive(Debug, Clone, PartialEq)]
pub struct NormalizeConfig {
    /// Fields to normalize (lowercase keys, trim whitespace from values)
    pub fields: Vec<String>,
    /// Whether to lowercase field names
    pub lowercase_keys: bool,
    /// Whether to trim whitespace from string values
    pub trim_values: bool,
    /// Whether to normalize all fields (overrides `fields`)
    pub all_fields: bool,
}

impl Default for NormalizeConfig {
    fn default() -> Self {
        Self {
            fields: vec![],
            lowercase_keys: true,
            trim_values: true,
            all_fields: false,
        }
    }
}

impl NormalizeConfig {
    pub fn new(fields: Vec<String>) -> Self {
        Self {
            fields,
            ..Default::default()
        }
    }

    pub fn all() -> Self {
        Self {
            all_fields: true,
            ..Default::default()
        }
    }

    pub fn with_lowercase_keys(mut self, v: bool) -> Self {
        self.lowercase_keys = v;
        self
    }

    pub fn with_trim_values(mut self, v: bool) -> Self {
        self.trim_values = v;
        self
    }
}

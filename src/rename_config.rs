use std::collections::HashMap;

/// Configuration for the field rename operation.
/// Maps original field names to their new names.
#[derive(Debug, Clone, PartialEq)]
pub struct RenameConfig {
    /// Map of old_name -> new_name
    pub mappings: HashMap<String, String>,
}

impl RenameConfig {
    pub fn new(mappings: HashMap<String, String>) -> Self {
        Self { mappings }
    }

    /// Build a RenameConfig from a list of (from, to) string pairs.
    pub fn from_pairs(pairs: Vec<(&str, &str)>) -> Self {
        let mappings = pairs
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        Self { mappings }
    }

    /// Returns true if no rename mappings are defined.
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }
}

impl Default for RenameConfig {
    fn default() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }
}

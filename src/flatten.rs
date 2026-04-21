use std::collections::HashMap;
use serde_json::{Value, Map};

/// Configuration for flattening nested JSON log fields
#[derive(Debug, Clone)]
pub struct FlattenConfig {
    /// Separator used between nested keys (e.g. "." produces "a.b.c")
    pub separator: String,
    /// Maximum depth to flatten (None = unlimited)
    pub max_depth: Option<usize>,
    /// Prefix to prepend to all flattened keys
    pub prefix: Option<String>,
}

impl Default for FlattenConfig {
    fn default() -> Self {
        FlattenConfig {
            separator: ".".to_string(),
            max_depth: None,
            prefix: None,
        }
    }
}

/// Flatten a nested JSON object into a single-level map of dot-separated keys.
pub fn flatten_value(value: &Value, config: &FlattenConfig) -> HashMap<String, Value> {
    let mut result = HashMap::new();
    let prefix = config.prefix.clone().unwrap_or_default();
    flatten_recursive(value, &prefix, config, 0, &mut result);
    result
}

fn flatten_recursive(
    value: &Value,
    prefix: &str,
    config: &FlattenConfig,
    depth: usize,
    result: &mut HashMap<String, Value>,
) {
    let at_max_depth = config.max_depth.map(|max| depth >= max).unwrap_or(false);

    match value {
        Value::Object(map) if !at_max_depth => {
            for (k, v) in map {
                let new_key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}{}{}", prefix, config.separator, k)
                };
                flatten_recursive(v, &new_key, config, depth + 1, result);
            }
        }
        _ => {
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), value.clone());
            }
        }
    }
}

/// Flatten a JSON object map directly.
pub fn flatten_map(map: &Map<String, Value>, config: &FlattenConfig) -> HashMap<String, Value> {
    flatten_value(&Value::Object(map.clone()), config)
}

use std::collections::HashMap;
use serde_json::{Value, Map};
use crate::pivot_config::PivotConfig;
use crate::log_entry::LogEntry;

/// Pivot log entries by grouping on a key field and aggregating a value field.
/// Produces one output entry per unique key value, with counts or collected values.
pub struct Pivot {
    config: PivotConfig,
}

impl Pivot {
    pub fn new(config: PivotConfig) -> Self {
        Self { config }
    }

    pub fn apply(&self, entries: &[LogEntry]) -> Vec<LogEntry> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for entry in entries {
            let key_val = entry
                .fields
                .get(&self.config.key_field)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "(missing)".to_string());

            let value_val = entry
                .fields
                .get(&self.config.value_field)
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .unwrap_or_default();

            groups.entry(key_val).or_default().push(value_val);
        }

        let mut result: Vec<LogEntry> = groups
            .into_iter()
            .map(|(key, values)| {
                let mut map = Map::new();
                map.insert(self.config.key_field.clone(), Value::String(key));
                map.insert(
                    "count".to_string(),
                    Value::Number(values.len().into()),
                );
                if self.config.collect_values {
                    map.insert(
                        "values".to_string(),
                        Value::Array(values.into_iter().map(Value::String).collect()),
                    );
                }
                LogEntry {
                    raw: String::new(),
                    fields: Value::Object(map),
                    timestamp: None,
                }
            })
            .collect();

        result.sort_by(|a, b| {
            let ka = a.fields.get(&self.config.key_field).and_then(Value::as_str).unwrap_or("");
            let kb = b.fields.get(&self.config.key_field).and_then(Value::as_str).unwrap_or("");
            ka.cmp(kb)
        });

        result
    }
}

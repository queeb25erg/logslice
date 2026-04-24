use std::collections::HashMap;
use serde_json::Value;
use crate::log_entry::LogEntry;
use crate::group_config::GroupConfig;

/// Groups log entries by one or more field values.
pub struct Grouper {
    config: GroupConfig,
}

impl Grouper {
    pub fn new(config: GroupConfig) -> Self {
        Self { config }
    }

    /// Groups a slice of log entries into a map keyed by composite group key.
    pub fn group<'a>(&self, entries: &'a [LogEntry]) -> HashMap<String, Vec<&'a LogEntry>> {
        let mut map: HashMap<String, Vec<&'a LogEntry>> = HashMap::new();
        for entry in entries {
            let key = self.build_key(entry);
            map.entry(key).or_default().push(entry);
        }
        map
    }

    /// Builds a composite string key from the configured fields.
    fn build_key(&self, entry: &LogEntry) -> String {
        self.config
            .fields
            .iter()
            .map(|field| {
                entry
                    .fields
                    .get(field)
                    .map(|v| match v {
                        Value::String(s) => s.clone(),
                        other => other.to_string(),
                    })
                    .unwrap_or_else(|| self.config.missing_value.clone())
            })
            .collect::<Vec<_>>()
            .join(&self.config.separator)
    }

    /// Returns the configured group fields.
    pub fn fields(&self) -> &[String] {
        &self.config.fields
    }
}

use crate::log_entry::LogEntry;
use crate::correlate_config::CorrelateConfig;
use std::collections::HashMap;

/// Groups log entries by a correlation key and links related entries together.
pub struct Correlator {
    config: CorrelateConfig,
}

#[derive(Debug, Clone)]
pub struct CorrelatedGroup {
    pub key: String,
    pub entries: Vec<LogEntry>,
}

impl Correlator {
    pub fn new(config: CorrelateConfig) -> Self {
        Self { config }
    }

    /// Correlate a slice of log entries by the configured field.
    /// Returns groups of entries sharing the same correlation key value.
    pub fn correlate(&self, entries: &[LogEntry]) -> Vec<CorrelatedGroup> {
        let mut map: HashMap<String, Vec<LogEntry>> = HashMap::new();

        for entry in entries {
            if let Some(value) = entry.fields.get(&self.config.field) {
                let key = value.to_string();
                map.entry(key).or_default().push(entry.clone());
            } else if self.config.include_unkeyed {
                map.entry(String::from("__unkeyed__"))
                    .or_default()
                    .push(entry.clone());
            }
        }

        let mut groups: Vec<CorrelatedGroup> = map
            .into_iter()
            .filter(|(_, entries)| entries.len() >= self.config.min_group_size)
            .map(|(key, mut entries)| {
                if self.config.sort_within_group {
                    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
                }
                CorrelatedGroup { key, entries }
            })
            .collect();

        groups.sort_by(|a, b| a.key.cmp(&b.key));
        groups
    }

    /// Flatten correlated groups back into a single ordered list,
    /// optionally annotating each entry with its group key.
    pub fn flatten(&self, groups: &[CorrelatedGroup]) -> Vec<LogEntry> {
        let mut result = Vec::new();
        for group in groups {
            for entry in &group.entries {
                let mut e = entry.clone();
                if self.config.annotate_key {
                    e.fields.insert(
                        self.config.annotation_field.clone(),
                        serde_json::Value::String(group.key.clone()),
                    );
                }
                result.push(e);
            }
        }
        result
    }
}

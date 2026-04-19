use std::collections::HashMap;
use crate::log_entry::LogEntry;

#[derive(Debug, Clone)]
pub struct AggregateConfig {
    pub group_by: String,
    pub count: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AggregateResult {
    pub key: String,
    pub count: usize,
}

pub struct Aggregator {
    config: AggregateConfig,
    counts: HashMap<String, usize>,
}

impl Aggregator {
    pub fn new(config: AggregateConfig) -> Self {
        Self {
            config,
            counts: HashMap::new(),
        }
    }

    pub fn process(&mut self, entry: &LogEntry) {
        let key = entry
            .fields
            .get(&self.config.group_by)
            .cloned()
            .unwrap_or_else(|| "(unknown)".to_string());
        *self.counts.entry(key).or_insert(0) += 1;
    }

    pub fn results(&self) -> Vec<AggregateResult> {
        let mut results: Vec<AggregateResult> = self
            .counts
            .iter()
            .map(|(k, &v)| AggregateResult {
                key: k.clone(),
                count: v,
            })
            .collect();
        results.sort_by(|a, b| b.count.cmp(&a.count).then(a.key.cmp(&b.key)));
        results
    }

    pub fn reset(&mut self) {
        self.counts.clear();
    }
}

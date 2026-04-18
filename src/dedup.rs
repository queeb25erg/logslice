use crate::log_entry::LogEntry;

/// Deduplication strategy for log entries
#[derive(Debug, Clone, PartialEq)]
pub enum DedupStrategy {
    /// Deduplicate by exact message match
    ExactMessage,
    /// Deduplicate by a specific field value
    ByField(String),
    /// Deduplicate consecutive identical entries only
    Consecutive,
}

/// Deduplicates log entries based on a chosen strategy
pub struct Deduplicator {
    strategy: DedupStrategy,
    seen: std::collections::HashSet<String>,
    last_key: Option<String>,
}

impl Deduplicator {
    pub fn new(strategy: DedupStrategy) -> Self {
        Self {
            strategy,
            seen: std::collections::HashSet::new(),
            last_key: None,
        }
    }

    /// Returns true if the entry should be kept (not a duplicate)
    pub fn is_unique(&mut self, entry: &LogEntry) -> bool {
        let key = self.extract_key(entry);
        match self.strategy {
            DedupStrategy::Consecutive => {
                let is_dup = self.last_key.as_deref() == Some(&key);
                self.last_key = Some(key);
                !is_dup
            }
            _ => self.seen.insert(key),
        }
    }

    fn extract_key(&self, entry: &LogEntry) -> String {
        match &self.strategy {
            DedupStrategy::ExactMessage | DedupStrategy::Consecutive => {
                entry.message.clone()
            }
            DedupStrategy::ByField(field) => {
                entry.fields
                    .get(field)
                    .cloned()
                    .unwrap_or_default()
            }
        }
    }

    pub fn filter(&mut self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().filter(|e| self.is_unique(e)).collect()
    }
}

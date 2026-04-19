/// Provides surrounding log lines as context around matched entries.
use crate::log_entry::LogEntry;

#[derive(Debug, Clone)]
pub struct ContextConfig {
    pub before: usize,
    pub after: usize,
}

impl ContextConfig {
    pub fn new(before: usize, after: usize) -> Self {
        Self { before, after }
    }
}

/// Given all entries and a set of matched indices, return entries with context.
pub fn apply_context(entries: &[LogEntry], matched: &[usize], config: &ContextConfig) -> Vec<LogEntry> {
    if matched.is_empty() {
        return vec![];
    }

    let mut indices: Vec<usize> = Vec::new();

    for &idx in matched {
        let start = idx.saturating_sub(config.before);
        let end = (idx + config.after + 1).min(entries.len());
        for i in start..end {
            indices.push(i);
        }
    }

    indices.sort_unstable();
    indices.dedup();

    indices.into_iter().map(|i| entries[i].clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
}

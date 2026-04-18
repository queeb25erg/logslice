use crate::field_filter::FieldFilter;
use crate::log_entry::LogEntry;

/// A chain of field filters that must ALL match (AND semantics).
#[derive(Debug, Default)]
pub struct FieldFilterChain {
    filters: Vec<FieldFilter>,
}

impl FieldFilterChain {
    pub fn new() -> Self {
        FieldFilterChain { filters: Vec::new() }
    }

    pub fn add(&mut self, filter: FieldFilter) {
        self.filters.push(filter);
    }

    /// Build a chain from a slice of expression strings (e.g. `["level=error", "host!=web01"]`).
    pub fn from_exprs(exprs: &[String]) -> Result<Self, String> {
        let mut chain = FieldFilterChain::new();
        for expr in exprs {
            match FieldFilter::parse(expr) {
                Some(f) => chain.add(f),
                None => return Err(format!("Invalid filter expression: '{}'", expr)),
            }
        }
        Ok(chain)
    }

    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    /// Returns true if all filters match the entry (or the chain is empty).
    pub fn matches(&self, entry: &LogEntry) -> bool {
        self.filters.iter().all(|f| f.matches(entry))
    }
}

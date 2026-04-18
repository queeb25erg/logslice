use crate::error::{LogSliceError, Result};
use crate::log_entry::LogEntry;
use crate::filter::Filter;
use crate::field_filter_chain::FieldFilterChain;
use crate::time_range::TimeRange;

pub struct Pipeline {
    pub time_range: Option<TimeRange>,
    pub field_chain: Option<FieldFilterChain>,
    pub limit: Option<usize>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            time_range: None,
            field_chain: None,
            limit: None,
        }
    }

    pub fn with_time_range(mut self, tr: TimeRange) -> Self {
        self.time_range = Some(tr);
        self
    }

    pub fn with_field_chain(mut self, fc: FieldFilterChain) -> Self {
        self.field_chain = Some(fc);
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn run(&self, entries: Vec<LogEntry>) -> Result<Vec<LogEntry>> {
        if entries.is_empty() {
            return Ok(vec![]);
        }

        let mut results: Vec<LogEntry> = entries
            .into_iter()
            .filter(|entry| {
                let time_ok = self.time_range
                    .as_ref()
                    .map(|tr| tr.contains(entry.timestamp))
                    .unwrap_or(true);
                let field_ok = self.field_chain
                    .as_ref()
                    .map(|fc| fc.matches(entry))
                    .unwrap_or(true);
                time_ok && field_ok
            })
            .collect();

        if let Some(limit) = self.limit {
            results.truncate(limit);
        }

        Ok(results)
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

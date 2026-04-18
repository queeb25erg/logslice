use crate::log_entry::LogEntry;
use crate::time_range::TimeRange;
use crate::field_filter_chain::FieldFilterChain;

/// Applies time range and field filter chain to a collection of log entries,
/// returning only those that satisfy all constraints.
pub struct Slicer {
    pub time_range: Option<TimeRange>,
    pub field_chain: FieldFilterChain,
}

impl Slicer {
    pub fn new(time_range: Option<TimeRange>, field_chain: FieldFilterChain) -> Self {
        Self { time_range, field_chain }
    }

    pub fn apply<'a>(&self, entries: &'a [LogEntry]) -> Vec<&'a LogEntry> {
        entries
            .iter()
            .filter(|e| self.matches(e))
            .collect()
    }

    fn matches(&self, entry: &LogEntry) -> bool {
        if let Some(ref tr) = self.time_range {
            match entry.timestamp {
                Some(ts) => {
                    if !tr.contains(ts) {
                        return false;
                    }
                }
                None => return false,
            }
        }
        self.field_chain.matches(entry)
    }
}

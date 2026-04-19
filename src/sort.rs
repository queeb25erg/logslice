use crate::log_entry::LogEntry;

#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone)]
pub struct SortConfig {
    pub field: String,
    pub order: SortOrder,
}

impl SortConfig {
    pub fn new(field: impl Into<String>, order: SortOrder) -> Self {
        Self { field: field.into(), order }
    }

    pub fn ascending(field: impl Into<String>) -> Self {
        Self::new(field, SortOrder::Ascending)
    }

    pub fn descending(field: impl Into<String>) -> Self {
        Self::new(field, SortOrder::Descending)
    }
}

pub fn sort_entries(entries: &mut Vec<LogEntry>, config: &SortConfig) {
    entries.sort_by(|a, b| {
        let va = a.fields.get(&config.field).map(|s| s.as_str()).unwrap_or("");
        let vb = b.fields.get(&config.field).map(|s| s.as_str()).unwrap_or("");
        let cmp = va.cmp(vb);
        match config.order {
            SortOrder::Ascending => cmp,
            SortOrder::Descending => cmp.reverse(),
        }
    });
}

pub fn sort_by_timestamp(entries: &mut Vec<LogEntry>, order: &SortOrder) {
    entries.sort_by(|a, b| {
        let cmp = a.timestamp.cmp(&b.timestamp);
        match order {
            SortOrder::Ascending => cmp,
            SortOrder::Descending => cmp.reverse(),
        }
    });
}

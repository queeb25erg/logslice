use crate::log_entry::LogEntry;

#[derive(Debug, Clone)]
pub struct TruncateConfig {
    pub max_field_len: usize,
    pub fields: Option<Vec<String>>,
    pub suffix: String,
}

impl Default for TruncateConfig {
    fn default() -> Self {
        Self {
            max_field_len: 256,
            fields: None,
            suffix: "...".to_string(),
        }
    }
}

pub fn truncate_entry(entry: &mut LogEntry, config: &TruncateConfig) {
    for (key, value) in entry.fields.iter_mut() {
        let should_truncate = config
            .fields
            .as_ref()
            .map(|f| f.iter().any(|k| k == key))
            .unwrap_or(true);

        if should_truncate && value.len() > config.max_field_len {
            let mut truncated = value[..config.max_field_len].to_string();
            truncated.push_str(&config.suffix);
            *value = truncated;
        }
    }
}

pub fn truncate_entries(entries: Vec<LogEntry>, config: &TruncateConfig) -> Vec<LogEntry> {
    entries
        .into_iter()
        .map(|mut e| {
            truncate_entry(&mut e, config);
            e
        })
        .collect()
}

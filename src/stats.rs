use std::collections::HashMap;
use crate::log_entry::LogEntry;

#[derive(Debug, Default)]
pub struct LogStats {
    pub total: usize,
    pub by_level: HashMap<String, usize>,
    pub earliest: Option<String>,
    pub latest: Option<String>,
}

impl LogStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, entry: &LogEntry) {
        self.total += 1;

        if let Some(level) = entry.fields.get("level") {
            *self.by_level.entry(level.clone()).or_insert(0) += 1;
        }

        if let Some(ts) = &entry.timestamp {
            match (&self.earliest, &self.latest) {
                (None, _) => {
                    self.earliest = Some(ts.clone());
                    self.latest = Some(ts.clone());
                }
                (Some(e), Some(l)) => {
                    if ts < e {
                        self.earliest = Some(ts.clone());
                    }
                    if ts > l {
                        self.latest = Some(ts.clone());
                    }
                }
                _ => {}
            }
        }
    }

    pub fn summary(&self) -> String {
        let mut lines = vec![
            format!("Total entries : {}", self.total),
        ];
        if let Some(e) = &self.earliest {
            lines.push(format!("Earliest      : {}", e));
        }
        if let Some(l) = &self.latest {
            lines.push(format!("Latest        : {}", l));
        }
        if !self.by_level.is_empty() {
            lines.push("By level:".to_string());
            let mut levels: Vec<_> = self.by_level.iter().collect();
            levels.sort_by_key(|(k, _)| k.clone());
            for (lvl, count) in levels {
                lines.push(format!("  {:10}: {}", lvl, count));
            }
        }
        lines.join("\n")
    }
}

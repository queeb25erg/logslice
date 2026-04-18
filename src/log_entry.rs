use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: Option<DateTime<Utc>>,
    pub raw: String,
    pub fields: HashMap<String, Value>,
}

impl LogEntry {
    pub fn from_json_line(line: &str) -> Option<Self> {
        let value: Value = serde_json::from_str(line).ok()?;
        let obj = value.as_object()?;

        let mut fields = HashMap::new();
        for (k, v) in obj {
            fields.insert(k.clone(), v.clone());
        }

        let timestamp = Self::extract_timestamp(&fields);

        Some(LogEntry {
            timestamp,
            raw: line.to_string(),
            fields,
        })
    }

    fn extract_timestamp(fields: &HashMap<String, Value>) -> Option<DateTime<Utc>> {
        let ts_keys = ["timestamp", "time", "ts", "@timestamp", "datetime"];
        for key in &ts_keys {
            if let Some(val) = fields.get(*key) {
                let s = match val {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    _ => continue,
                };
                if let Ok(dt) = crate::time_range::parse_timestamp(&s) {
                    return Some(dt);
                }
            }
        }
        None
    }

    pub fn get_field(&self, key: &str) -> Option<&Value> {
        self.fields.get(key)
    }

    pub fn field_matches(&self, key: &str, value: &str) -> bool {
        match self.get_field(key) {
            Some(Value::String(s)) => s == value,
            Some(Value::Number(n)) => n.to_string() == value,
            Some(Value::Bool(b)) => b.to_string() == value,
            _ => false,
        }
    }
}

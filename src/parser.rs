use crate::log_entry::LogEntry;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyLine,
    InvalidJson(String),
    MissingTimestamp,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyLine => write!(f, "empty line"),
            ParseError::InvalidJson(msg) => write!(f, "invalid JSON: {}", msg),
            ParseError::MissingTimestamp => write!(f, "missing timestamp field"),
        }
    }
}

pub fn parse_line(line: &str) -> Result<LogEntry, ParseError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(ParseError::EmptyLine);
    }

    let value: serde_json::Value = serde_json::from_str(trimmed)
        .map_err(|e| ParseError::InvalidJson(e.to_string()))?;

    let obj = value.as_object().ok_or_else(|| {
        ParseError::InvalidJson("expected a JSON object".to_string())
    })?;

    let timestamp = obj
        .get("timestamp")
        .or_else(|| obj.get("ts"))
        .or_else(|| obj.get("time"))
        .and_then(|v| v.as_str())
        .ok_or(ParseError::MissingTimestamp)?
        .to_string();

    let mut fields: HashMap<String, String> = HashMap::new();
    for (k, v) in obj {
        if k == "timestamp" || k == "ts" || k == "time" {
            continue;
        }
        let val_str = match v {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        fields.insert(k.clone(), val_str);
    }

    Ok(LogEntry { timestamp, fields })
}

pub fn parse_lines<'a>(
    lines: impl Iterator<Item = &'a str>,
    skip_errors: bool,
) -> Vec<LogEntry> {
    let mut entries = Vec::new();
    for line in lines {
        match parse_line(line) {
            Ok(entry) => entries.push(entry),
            Err(ParseError::EmptyLine) => {}
            Err(e) => {
                if !skip_errors {
                    eprintln!("logslice: parse warning: {}", e);
                }
            }
        }
    }
    entries
}

use crate::log_entry::LogEntry;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    Plain,
    Csv,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(OutputFormat::Json),
            "plain" | "text" => Some(OutputFormat::Plain),
            "csv" => Some(OutputFormat::Csv),
            _ => None,
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Plain => write!(f, "plain"),
            OutputFormat::Csv => write!(f, "csv"),
        }
    }
}

pub fn format_entry(entry: &LogEntry, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Json => format_json(entry),
        OutputFormat::Plain => format_plain(entry),
        OutputFormat::Csv => format_csv(entry),
    }
}

fn format_json(entry: &LogEntry) -> String {
    let fields: Vec<String> = entry
        .fields
        .iter()
        .map(|(k, v)| format!("\"{}\": \"{}\"", k, v))
        .collect();
    let fields_str = fields.join(", ");
    format!(
        "{{\"timestamp\": \"{}\", \"level\": \"{}\", \"message\": \"{}\"{}}}",
        entry.timestamp,
        entry.level,
        entry.message,
        if fields_str.is_empty() { String::new() } else { format!(", {}", fields_str) }
    )
}

fn format_plain(entry: &LogEntry) -> String {
    format!("[{}] {} - {}", entry.timestamp, entry.level, entry.message)
}

fn format_csv(entry: &LogEntry) -> String {
    format!("{},{},{}", entry.timestamp, entry.level, escape_csv(&entry.message))
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

pub fn csv_header() -> &'static str {
    "timestamp,level,message"
}

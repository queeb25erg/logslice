//! Field value normalization for log entries.
//!
//! Provides functionality to normalize field values in log entries,
//! such as lowercasing strings, trimming whitespace, or standardizing
//! boolean/numeric representations.

use crate::log_entry::LogEntry;
use crate::normalize_config::{NormalizeConfig, NormalizeOp};
use serde_json::Value;

/// Normalizes fields in a log entry according to the provided configuration.
pub struct Normalizer {
    config: NormalizeConfig,
}

impl Normalizer {
    /// Creates a new `Normalizer` with the given configuration.
    pub fn new(config: NormalizeConfig) -> Self {
        Self { config }
    }

    /// Applies normalization operations to the given log entry.
    /// Returns a new `LogEntry` with normalized field values.
    pub fn normalize(&self, mut entry: LogEntry) -> LogEntry {
        for (field, op) in &self.config.fields {
            if let Some(value) = entry.fields.get_mut(field) {
                *value = apply_op(value.clone(), op);
            }
        }
        entry
    }
}

/// Applies a single normalization operation to a JSON value.
fn apply_op(value: Value, op: &NormalizeOp) -> Value {
    match op {
        NormalizeOp::Lowercase => match value {
            Value::String(s) => Value::String(s.to_lowercase()),
            other => other,
        },
        NormalizeOp::Uppercase => match value {
            Value::String(s) => Value::String(s.to_uppercase()),
            other => other,
        },
        NormalizeOp::Trim => match value {
            Value::String(s) => Value::String(s.trim().to_string()),
            other => other,
        },
        NormalizeOp::ToBool => match &value {
            Value::String(s) => {
                let lower = s.trim().to_lowercase();
                match lower.as_str() {
                    "true" | "yes" | "1" | "on" => Value::Bool(true),
                    "false" | "no" | "0" | "off" => Value::Bool(false),
                    _ => value,
                }
            }
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Bool(i != 0)
                } else {
                    value
                }
            }
            other => other.clone(),
        },
        NormalizeOp::ToNumber => match &value {
            Value::String(s) => {
                let trimmed = s.trim();
                if let Ok(i) = trimmed.parse::<i64>() {
                    Value::Number(i.into())
                } else if let Ok(f) = trimmed.parse::<f64>() {
                    serde_json::Number::from_f64(f)
                        .map(Value::Number)
                        .unwrap_or(value)
                } else {
                    value
                }
            }
            other => other.clone(),
        },
        NormalizeOp::ToString => match &value {
            Value::Bool(b) => Value::String(b.to_string()),
            Value::Number(n) => Value::String(n.to_string()),
            Value::Null => Value::String("null".to_string()),
            other => other.clone(),
        },
    }
}

#[cfg(test)]
#[path = "normalize_tests.rs"]
mod tests;

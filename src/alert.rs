//! Alert module for triggering notifications when log conditions are met.
//!
//! Supports threshold-based alerts, pattern matching alerts, and rate-based
//! alerts with configurable cooldown periods to prevent alert flooding.

use crate::log_entry::LogEntry;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Defines the condition that triggers an alert.
#[derive(Debug, Clone)]
pub enum AlertCondition {
    /// Triggers when a field matches a specific value.
    FieldEquals { field: String, value: String },
    /// Triggers when a field contains a substring.
    FieldContains { field: String, pattern: String },
    /// Triggers when the count of matching entries exceeds a threshold within a window.
    CountExceeds { count: usize, window_secs: u64 },
    /// Triggers when a severity level meets or exceeds the given level.
    SeverityAtLeast { level: String },
}

/// Configuration for an alert rule.
#[derive(Debug, Clone)]
pub struct AlertConfig {
    /// Human-readable name for the alert.
    pub name: String,
    /// The condition that triggers the alert.
    pub condition: AlertCondition,
    /// Minimum time between repeated alerts (cooldown).
    pub cooldown: Duration,
    /// Optional message template for the alert output.
    pub message_template: Option<String>,
}

/// Represents a fired alert event.
#[derive(Debug, Clone)]
pub struct AlertEvent {
    /// Name of the alert rule that fired.
    pub rule_name: String,
    /// The log entry that triggered the alert.
    pub entry: LogEntry,
    /// Rendered alert message.
    pub message: String,
}

/// Evaluates log entries against alert rules and fires alerts when conditions are met.
pub struct AlertEngine {
    rules: Vec<AlertConfig>,
    /// Tracks last fire time per rule name for cooldown enforcement.
    last_fired: HashMap<String, Instant>,
    /// Sliding window entry counts per rule name.
    window_counts: HashMap<String, Vec<Instant>>,
}

const SEVERITY_ORDER: &[&str] = &["debug", "info", "warn", "error", "fatal"];

fn severity_rank(level: &str) -> usize {
    let lower = level.to_lowercase();
    SEVERITY_ORDER
        .iter()
        .position(|&s| s == lower.as_str())
        .unwrap_or(0)
}

impl AlertEngine {
    /// Creates a new `AlertEngine` with the given alert rules.
    pub fn new(rules: Vec<AlertConfig>) -> Self {
        Self {
            rules,
            last_fired: HashMap::new(),
            window_counts: HashMap::new(),
        }
    }

    /// Evaluates a log entry against all rules and returns any fired alerts.
    pub fn evaluate(&mut self, entry: &LogEntry) -> Vec<AlertEvent> {
        let mut events = Vec::new();
        let now = Instant::now();

        for rule in &self.rules {
            let triggered = match &rule.condition {
                AlertCondition::FieldEquals { field, value } => entry
                    .fields
                    .get(field)
                    .map(|v| v == value)
                    .unwrap_or(false),

                AlertCondition::FieldContains { field, pattern } => entry
                    .fields
                    .get(field)
                    .map(|v| v.contains(pattern.as_str()))
                    .unwrap_or(false),

                AlertCondition::SeverityAtLeast { level } => {
                    let entry_rank = entry
                        .fields
                        .get("level")
                        .or_else(|| entry.fields.get("severity"))
                        .map(|v| severity_rank(v))
                        .unwrap_or(0);
                    entry_rank >= severity_rank(level)
                }

                AlertCondition::CountExceeds { count, window_secs } => {
                    let window = Duration::from_secs(*window_secs);
                    let timestamps = self
                        .window_counts
                        .entry(rule.name.clone())
                        .or_default();
                    timestamps.push(now);
                    // Prune entries outside the window.
                    timestamps.retain(|t| now.duration_since(*t) <= window);
                    timestamps.len() > *count
                }
            };

            if !triggered {
                continue;
            }

            // Enforce cooldown.
            if let Some(last) = self.last_fired.get(&rule.name) {
                if now.duration_since(*last) < rule.cooldown {
                    continue;
                }
            }

            self.last_fired.insert(rule.name.clone(), now);

            let message = rule
                .message_template
                .as_deref()
                .unwrap_or("Alert triggered: {name}")
                .replace("{name}", &rule.name)
                .replace(
                    "{message}",
                    entry.fields.get("message").map(|s| s.as_str()).unwrap_or(""),
                );

            events.push(AlertEvent {
                rule_name: rule.name.clone(),
                entry: entry.clone(),
                message,
            });
        }

        events
    }
}

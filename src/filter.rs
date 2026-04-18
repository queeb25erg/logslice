use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
serde_json::Value;

pub struct FilterConfig {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub field_key: Option<String>,
    pub field_value: Option<String>,
    pub time_field: String,
}

impl FilterConfig {
    pub fn new(
        from: Option<&str>,
        to: Option<&str>,
        field: Option<&str>,
        time_field: &str,
    ) -> Result<Self> {
        let from = from.map(|s| s.parse::<DateTime<Utc>>()).transpose()
            .map_err(|e| anyhow!("Invalid --from datetime: {}", e))?;
        let to = to.map(|s| s.parse::<DateTime<Utc>>()).transpose()
            .map_err(|e| anyhow!("Invalid --to datetime: {}", e))?;

        let (field_key, field_value) = match field {
            Some(f) => {
                let parts: Vec<&str> = f.splitn(2, '=').collect();
                if parts.len() != 2 {
                    return Err(anyhow!("--field must be in key=value format"));
                }
                (Some(parts[0].to_string()), Some(parts[1].to_string()))
            }
            None => (None, None),
        };

        Ok(Self { from, to, field_key, field_value, time_field: time_field.to_string() })
    }
}

pub fn apply_filter(line: &str, config: &FilterConfig) -> Result<bool> {
    let value: serde_json::Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(_) => return Ok(false),
    };

    if config.from.is_some() || config.to.is_some() {
        let ts_str = value[&config.time_field]
            .as_str()
            .ok_or_else(|| anyhow!("Missing or non-string timestamp field '{}'", config.time_field))?;
        let ts = ts_str.parse::<DateTime<Utc>>()
            .map_err(|e| anyhow!("Invalid timestamp '{}': {}", ts_str, e))?;

        if let Some(from) = config.from {
            if ts < from { return Ok(false); }
        }
        if let Some(to) = config.to {
            if ts > to { return Ok(false); }
        }
    }

    if let (Some(key), Some(expected)) = (&config.field_key, &config.field_value) {
        let actual = match &value[key] {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        if &actual != expected {
            return Ok(false);
        }
    }

    Ok(true)
}

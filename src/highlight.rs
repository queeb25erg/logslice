use std::collections::HashSet;

/// Highlights specific fields or values in a log line for terminal output.
#[derive(Debug, Clone)]
pub struct Highlighter {
    pub fields: HashSet<String>,
    pub color_code: String,
}

impl Highlighter {
    pub fn new(fields: Vec<String>) -> Self {
        Self {
            fields: fields.into_iter().collect(),
            color_code: "\x1b[33m".to_string(), // yellow
        }
    }

    pub fn with_color(mut self, code: &str) -> Self {
        self.color_code = code.to_string();
        self
    }

    /// Highlight matching key=value pairs in a raw log line.
    pub fn apply(&self, line: &str) -> String {
        if self.fields.is_empty() {
            return line.to_string();
        }
        let reset = "\x1b[0m";
        let mut result = line.to_string();
        for field in &self.fields {
            let pattern = format!("{}=", field);
            if let Some(start) = result.find(&pattern) {
                let end = result[start..]
                    .find(|c: char| c == ' ' || c == ',')
                    .map(|i| start + i)
                    .unwrap_or(result.len());
                let highlighted = format!(
                    "{}{}{}",
                    self.color_code,
                    &result[start..end],
                    reset
                );
                result = format!("{}{}{}", &result[..start], highlighted, &result[end..]);
            }
        }
        result
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new(vec![])
    }
}

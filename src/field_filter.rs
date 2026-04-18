use crate::log_entry::LogEntry;

#[derive(Debug, Clone, PartialEq)]
pub enum FieldOp {
    Equals,
    Contains,
    NotEquals,
}

#[derive(Debug, Clone)]
pub struct FieldFilter {
    pub field: String,
    pub op: FieldOp,
    pub value: String,
}

impl FieldFilter {
    pub fn new(field: impl Into<String>, op: FieldOp, value: impl Into<String>) -> Self {
        FieldFilter {
            field: field.into(),
            op,
            value: value.into(),
        }
    }

    /// Parse a filter expression like `level=error`, `msg~timeout`, `host!=web01`
    pub fn parse(expr: &str) -> Option<Self> {
        if let Some(idx) = expr.find("!=") {
            return Some(FieldFilter::new(&expr[..idx], FieldOp::NotEquals, &expr[idx + 2..]));
        }
        if let Some(idx) = expr.find('~') {
            return Some(FieldFilter::new(&expr[..idx], FieldOp::Contains, &expr[idx + 1..]));
        }
        if let Some(idx) = expr.find('=') {
            return Some(FieldFilter::new(&expr[..idx], FieldOp::Equals, &expr[idx + 1..]));
        }
        None
    }

    pub fn matches(&self, entry: &LogEntry) -> bool {
        match entry.fields.get(&self.field) {
            None => false,
            Some(val) => match self.op {
                FieldOp::Equals => val == &self.value,
                FieldOp::NotEquals => val != &self.value,
                FieldOp::Contains => val.contains(&self.value as &str),
            },
        }
    }
}

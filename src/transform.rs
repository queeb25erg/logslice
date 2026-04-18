use crate::log_entry::LogEntry;
use crate::error::AppError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TransformOp {
    RenameField { from: String, to: String },
    AddField { key: String, value: String },
    RemoveField { key: String },
    MaskField { key: String, mask: String },
}

#[derive(Debug, Clone, Default)]
pub struct Transformer {
    ops: Vec<TransformOp>,
}

impl Transformer {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    pub fn add_op(&mut self, op: TransformOp) {
        self.ops.push(op);
    }

    pub fn apply(&self, mut entry: LogEntry) -> Result<LogEntry, AppError> {
        for op in &self.ops {
            match op {
                TransformOp::RenameField { from, to } => {
                    if let Some(val) = entry.fields.remove(from) {
                        entry.fields.insert(to.clone(), val);
                    }
                }
                TransformOp::AddField { key, value } => {
                    entry.fields.insert(key.clone(), value.clone());
                }
                TransformOp::RemoveField { key } => {
                    entry.fields.remove(key);
                }
                TransformOp::MaskField { key, mask } => {
                    if entry.fields.contains_key(key) {
                        entry.fields.insert(key.clone(), mask.clone());
                    }
                }
            }
        }
        Ok(entry)
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}

pub fn parse_transform_op(s: &str) -> Result<TransformOp, AppError> {
    let parts: Vec<&str> = s.splitn(3, ':').collect();
    match parts.as_slice() {
        ["rename", from, to] => Ok(TransformOp::RenameField {
            from: from.to_string(),
            to: to.to_string(),
        }),
        ["add", key, value] => Ok(TransformOp::AddField {
            key: key.to_string(),
            value: value.to_string(),
        }),
        ["remove", key, ..] => Ok(TransformOp::RemoveField {
            key: key.to_string(),
        }),
        ["mask", key, mask] => Ok(TransformOp::MaskField {
            key: key.to_string(),
            mask: mask.to_string(),
        }),
        _ => Err(AppError::InvalidTransformOp(s.to_string())),
    }
}

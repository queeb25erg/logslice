use crate::transform::{parse_transform_op, TransformOp, Transformer};
use crate::error::AppError;

#[derive(Debug, Default)]
pub struct TransformConfig {
    pub ops: Vec<String>,
}

impl TransformConfig {
    pub fn new(ops: Vec<String>) -> Self {
        Self { ops }
    }

    pub fn build_transformer(&self) -> Result<Transformer, AppError> {
        let mut transformer = Transformer::new();
        for raw in &self.ops {
            let op = parse_transform_op(raw)?;
            transformer.add_op(op);
        }
        Ok(transformer)
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AggregateConfig {
    pub group_by: String,
    pub count: bool,
    pub min_count: Option<usize>,
}

impl AggregateConfig {
    pub fn new(group_by: impl Into<String>) -> Self {
        Self {
            group_by: group_by.into(),
            count: true,
            min_count: None,
        }
    }

    pub fn with_min_count(mut self, min: usize) -> Self {
        self.min_count = Some(min);
        self
    }

    pub fn validate(&self) -> Result<(), AppError> {
        if self.group_by.trim().is_empty() {
            return Err(AppError::Config(
                "aggregate group_by field must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}

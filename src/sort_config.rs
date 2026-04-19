use crate::sort::SortOrder;

#[derive(Debug, Clone)]
pub struct SortConfigBuilder {
    pub field: Option<String>,
    pub order: SortOrder,
    pub by_timestamp: bool,
}

impl Default for SortConfigBuilder {
    fn default() -> Self {
        Self {
            field: None,
            order: SortOrder::Ascending,
            by_timestamp: false,
        }
    }
}

impl SortConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = order;
        self
    }

    pub fn by_timestamp(mut self) -> Self {
        self.by_timestamp = true;
        self
    }

    pub fn is_valid(&self) -> bool {
        self.by_timestamp || self.field.is_some()
    }
}

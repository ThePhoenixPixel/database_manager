pub use crate::types::{FilterOperator, OrderDirection};
use crate::types::value::Value;

#[derive(Debug, Clone)]
pub struct Filter {
    pub column: String,
    pub operator: FilterOperator,
    pub value: Option<Value>,
    pub values: Option<Vec<Value>>, // For IN and BETWEEN
}

impl Filter {
    pub fn eq(column: impl Into<String>, value: Value) -> Self {
        Self {
            column: column.into(),
            operator: FilterOperator::Equals,
            value: Some(value),
            values: None,
        }
    }

    pub fn not_eq(column: impl Into<String>, value: Value) -> Self {
        Self {
            column: column.into(),
            operator: FilterOperator::NotEquals,
            value: Some(value),
            values: None,
        }
    }

    pub fn gt(column: impl Into<String>, value: Value) -> Self {
        Self {
            column: column.into(),
            operator: FilterOperator::GreaterThan,
            value: Some(value),
            values: None,
        }
    }

    pub fn lt(column: impl Into<String>, value: Value) -> Self {
        Self {
            column: column.into(),
            operator: FilterOperator::LessThan,
            value: Some(value),
            values: None,
        }
    }

    pub fn like(column: impl Into<String>, pattern: String) -> Self {
        Self {
            column: column.into(),
            operator: FilterOperator::Like,
            value: Some(Value::Text(pattern)),
            values: None,
        }
    }

    pub fn is_null(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            operator: FilterOperator::IsNull,
            value: None,
            values: None,
        }
    }

    pub fn in_values(column: impl Into<String>, value: Vec<Value>) -> Self {
        Self {
            column: column.into(),
            operator: FilterOperator::In,
            value: None,
            values: Some(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueryFilters {
    pub filters: Vec<Filter>,
    pub order_by: Option<Vec<(String, OrderDirection)>>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl QueryFilters {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    pub fn add(mut self, filter: Filter) -> QueryFilters {
        self.filters.push(filter);
        self
    }
    
    pub fn add_filter(&mut self, filter: Filter) {
        self.filters.push(filter);
    }

    pub fn order_by(mut self, column: impl Into<String>, direction: OrderDirection) -> Self {
        if self.order_by.is_none() {
            self.order_by = Some(Vec::new());
        }
        self.order_by.as_mut().unwrap().push((column.into(), direction));
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl Default for QueryFilters {
    fn default() -> Self {
        Self::new()
    }
}
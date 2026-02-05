#[derive(Debug, Clone, PartialEq)]
pub enum ForeignKeyAction {
    NoAction,
    Restrict,
    Cascade,
    SetNull,
    SetDefault,
}

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub name: String,
    pub columns: Vec<String>,
    pub referenced_table: String,
    pub referenced_columns: Vec<String>,
    pub on_delete: ForeignKeyAction,
    pub on_update: ForeignKeyAction,
}

impl ForeignKey {
    pub fn new(
        name: impl Into<String>,
        columns: Vec<String>,
        referenced_table: impl Into<String>,
        referenced_columns: Vec<String>,
    ) -> Self {
        Self {
            name: name.into(),
            columns,
            referenced_table: referenced_table.into(),
            referenced_columns,
            on_delete: ForeignKeyAction::NoAction,
            on_update: ForeignKeyAction::NoAction,
        }
    }

    pub fn on_delete(mut self, action: ForeignKeyAction) -> Self {
        self.on_delete = action;
        self
    }

    pub fn on_update(mut self, action: ForeignKeyAction) -> Self {
        self.on_update = action;
        self
    }

    pub fn cascade_delete(mut self) -> Self {
        self.on_delete = ForeignKeyAction::Cascade;
        self
    }

    pub fn cascade_update(mut self) -> Self {
        self.on_update = ForeignKeyAction::Cascade;
        self
    }
}

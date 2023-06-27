use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Count {
    pub name: String,
    pub value: i64,
    pub tags: Vec<String>,
}

pub enum CounterAction {
    ToggleTheme,
    Increment(usize, i64),
    Decrement(usize, i64),
    AddCount(Count),
    SelectTags(Vec<String>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Store {
    pub dark_theme: bool,
    pub counts: Vec<Count>,
    pub tags: Vec<String>,
    pub selected_tags: Vec<String>,
}
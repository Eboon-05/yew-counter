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
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Store {
    pub dark_theme: bool,
    pub counts: Vec<Count>,
}
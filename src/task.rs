use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: u64, title: String) -> Self {
        Self { id, title, done: false }
    }

    pub fn format_row(&self) -> String {
        let mark = if self.done { "x" } else { " " };
        format!("[{mark}] {:>3}  {}", self.id, self.title)
    }
}

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub tag: Option<String>,
    pub due: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(id: u64, title: String, tag: Option<String>, due: Option<NaiveDate>) -> Self {
        Self {
            id,
            title,
            tag,
            due,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn is_done(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn is_overdue(&self, today: NaiveDate) -> bool {
        match (self.due, self.is_done()) {
            (Some(due), false) => due < today,
            _ => false,
        }
    }

    pub fn format_row(&self) -> String {
        let mark = if self.is_done() { "x" } else { " " };
        let tag = self
            .tag
            .as_deref()
            .map(|t| format!(" #{t}"))
            .unwrap_or_default();
        let due = self
            .due
            .map(|d| format!(" (due {d})"))
            .unwrap_or_default();
        format!("[{mark}] {:>3}  {}{}{}", self.id, self.title, tag, due)
    }

    pub fn matches(&self, query: &str) -> bool {
        let needle = query.to_lowercase();
        self.title.to_lowercase().contains(&needle)
            || self
                .tag
                .as_deref()
                .map(|t| t.to_lowercase().contains(&needle))
                .unwrap_or(false)
    }
}

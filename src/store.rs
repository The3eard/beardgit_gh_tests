use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    next_id: u64,
    tasks: Vec<Task>,

    #[serde(skip)]
    path: PathBuf,
}

impl Store {
    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            let mut store = Self::default();
            store.path = path.to_path_buf();
            return Ok(store);
        }
        let bytes = fs::read(path)
            .with_context(|| format!("reading task store at {}", path.display()))?;
        let mut store: Self = serde_json::from_slice(&bytes)
            .with_context(|| format!("parsing task store at {}", path.display()))?;
        store.path = path.to_path_buf();
        Ok(store)
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let body = serde_json::to_vec_pretty(self)?;
        fs::write(&self.path, body)
            .with_context(|| format!("writing task store to {}", self.path.display()))?;
        Ok(())
    }

    pub fn add(&mut self, title: String) -> &Task {
        self.next_id += 1;
        self.tasks.push(Task::new(self.next_id, title));
        self.tasks.last().expect("task just pushed")
    }

    pub fn list(&self, include_done: bool) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| include_done || !t.done)
            .collect()
    }

    pub fn mark_done(&mut self, id: u64) -> Result<&Task> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| anyhow!("no task with id {id}"))?;
        task.done = true;
        Ok(task)
    }

    pub fn remove(&mut self, id: u64) {
        self.tasks.retain(|t| t.id != id);
    }
}

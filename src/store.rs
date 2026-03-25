use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use chrono::{NaiveDate, Utc};
use directories::ProjectDirs;
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
    pub fn load_default() -> Result<Self> {
        let path = default_store_path()?;
        Self::load_from(&path)
    }

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

    pub fn add(
        &mut self,
        title: String,
        tag: Option<String>,
        due: Option<NaiveDate>,
    ) -> Result<&Task> {
        self.next_id += 1;
        let task = Task::new(self.next_id, title, tag, due);
        self.tasks.push(task);
        Ok(self.tasks.last().expect("task just pushed"))
    }

    pub fn list(&self, include_done: bool) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| include_done || !t.is_done())
            .collect()
    }

    pub fn mark_done(&mut self, id: u64) -> Result<&Task> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| anyhow!("no task with id {id}"))?;
        if task.completed_at.is_none() {
            task.completed_at = Some(Utc::now());
        }
        Ok(task)
    }

    pub fn remove(&mut self, id: u64) -> Result<()> {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        if self.tasks.len() == before {
            return Err(anyhow!("no task with id {id}"));
        }
        Ok(())
    }
}

fn default_store_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("dev", "tasklog", "tasklog")
        .ok_or_else(|| anyhow!("could not determine an OS-appropriate data directory"))?;
    Ok(dirs.data_dir().join("tasks.json"))
}

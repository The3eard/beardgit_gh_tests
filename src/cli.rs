use chrono::NaiveDate;
use clap::{Parser, Subcommand};

use crate::recurrence::Recurrence;

#[derive(Debug, Parser)]
#[command(name = "tasklog", version, about = "A tiny terminal task tracker.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a new task to the store.
    Add {
        /// One-line task title.
        title: String,
        /// Optional tag to group related tasks (e.g. `work`, `bug`).
        #[arg(long)]
        tag: Option<String>,
        /// Optional due date in YYYY-MM-DD form.
        #[arg(long)]
        due: Option<NaiveDate>,
        /// Optional recurrence: daily, weekly, or monthly.
        #[arg(long, value_parser = clap::value_parser!(Recurrence))]
        repeat: Option<Recurrence>,
    },
    /// List open tasks (or all tasks with --all).
    List {
        /// Include completed tasks in the output.
        #[arg(long)]
        all: bool,
        /// Filter by tag.
        #[arg(long)]
        tag: Option<String>,
    },
    /// Mark a task as done.
    Done {
        /// Task ID to complete.
        id: u64,
    },
    /// Remove a task entirely.
    Rm {
        /// Task ID to remove.
        id: u64,
    },
    /// Full-text search across task titles and tags.
    Search {
        /// Case-insensitive substring to search for.
        query: String,
    },
}

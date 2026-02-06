use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "tasklog", version, about = "A tiny terminal task tracker.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a new task to the store.
    Add { title: String },
    /// List open tasks (or all tasks with --all).
    List {
        #[arg(long)]
        all: bool,
    },
    /// Mark a task as done.
    Done { id: u64 },
    /// Remove a task entirely.
    Rm { id: u64 },
}

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use tasklog::cli::{Cli, Command};
use tasklog::store::Store;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = default_store_path();
    let mut store = Store::load_from(&path)?;

    match cli.command {
        Command::Add { title } => {
            let task = store.add(title);
            println!("added #{} {}", task.id, task.title);
        }
        Command::List { all } => {
            for task in store.list(all) {
                println!("{}", task.format_row());
            }
        }
        Command::Done { id } => {
            let task = store.mark_done(id)?;
            println!("done #{} {}", task.id, task.title);
        }
        Command::Rm { id } => {
            store.remove(id);
            println!("removed #{id}");
        }
    }

    store.save()?;
    Ok(())
}

fn default_store_path() -> PathBuf {
    let home = std::env::var_os("HOME").unwrap_or_else(|| ".".into());
    PathBuf::from(home).join(".tasklog.json")
}

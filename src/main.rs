use anyhow::Result;
use clap::Parser;

use tasklog::cli::{Cli, Command};
use tasklog::store::Store;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = Store::load_default()?;

    match cli.command {
        Command::Add { title, tag, due } => {
            let task = store.add(title, tag, due)?;
            println!("added #{} {}", task.id, task.title);
        }
        Command::List { all, tag } => {
            for task in store.list(all, tag.as_deref()) {
                println!("{}", task.format_row());
            }
        }
        Command::Done { id } => {
            let task = store.mark_done(id)?;
            println!("done #{} {}", task.id, task.title);
        }
        Command::Rm { id } => {
            store.remove(id)?;
            println!("removed #{id}");
        }
        Command::Search { query } => {
            for task in store.search(&query) {
                println!("{}", task.format_row());
            }
        }
    }

    store.save()?;
    Ok(())
}

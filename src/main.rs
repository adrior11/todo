// TODO: Allow the implementation of multi line todos, split by a character like ;
// TODO: Implement a backup solution of someone accidentally deletes their todos -> Merge conflict
// TODO: Implement user feedback on actions like add sort and rm
// TODO: Add enhanced sorting
mod cli;
mod todo;
mod utils;

use clap::Parser;
use anyhow::Result;
use cli::Cli;
use todo::TodoList;
use utils::get_todo_file_path;


/// Run the main application logic
fn run() -> Result<()> {
    let args = Cli::parse();

    let file_path = get_todo_file_path()?;

    let mut todo_list = TodoList::load_from_file(&file_path)?;

    // Handle CLI commands or default to listing todos
    match args.pattern {
        Some(pattern) => todo_list.handle_cli(pattern),
        None => todo_list.list(),
    }
   
    todo_list.save_to_file(&file_path)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

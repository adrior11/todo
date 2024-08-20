#[macro_use]
mod macros;
mod cli;
mod todo;
mod utils;
mod config;
mod render;

use clap::Parser;
use anyhow::Result;
use cli::Cli;
use todo::TodoList;
use utils::get_todo_file_path;

/// Run the main application logic.
///
/// This function handles the following tasks:
/// - Parses the command-line arguments using the `Cli` struct.
/// - Loads the todo list from the specified file.
/// - Executes the appropriate command based on the CLI input or lists todos by default.
/// - Saves the updated todo list back to the file.
///
/// # Returns
///
/// `Result<()>` - Returns `Ok(())` if the operations complete successfully, or an error if any step fails.
fn run() -> Result<()> {
    let args = Cli::parse();

    let file_path = get_todo_file_path()?;

    let mut todo_list = TodoList::load_from_file(&file_path)?;

    // Handle CLI commands or default to listing todos
    match args.pattern {
        Some(pattern) => todo_list.handle_cli(pattern)?,
        None => todo_list.list(),
    }
   
    todo_list.save_to_file(&file_path)?;

    Ok(())
}

/// The main entry point of the application.
///
/// This function calls the `run` function and handles any errors that occur by printing them
/// to stderr and exiting the process with a non-zero status code.
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

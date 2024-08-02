use clap::{Parser, Subcommand, ValueEnum};

/// CLI structure to parse command line arguments
#[derive(Parser)]
#[command(author, version, about = "A CLI todo application", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub pattern: Option<Pattern>,
}

/// Enum representing the different command patterns
#[derive(Subcommand)]
pub enum Pattern {
    /// Add a new todo
    Add { 
        /// The description of the todo(s), separated by '::' for multiple items
        #[arg(value_name = "TODO_DESCRIPTION", num_args(1..))]
        args: Vec<String> 
    },

    /// List all todos
    List,
    
    /// Mark a todo as done
    Done { 
        /// The ID of the todo to mark as done
        #[arg(value_name = "TODO_ID")]
        args: Vec<usize> 
    },

    /// Remove a todo
    Rm { 
        /// The ID of the todo to emove
        #[arg(value_name = "TODO_ID")]
        args: Vec<usize> 
    },

    /// Reset the todo list
    Reset {
        /// Include clearing the backup file
        #[arg(long)]
        include_backup: bool,
    },

    /// Sort todos
    Sort {
        /// The field to sort by (id, date, done)
        #[arg(value_name = "SORT_BY")]
        sort_by: Option<SortBy>,
    },
}

// TODO: Add docs 
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SortBy {
    Id,
    Date,
    Done, // Runs default sort 
}

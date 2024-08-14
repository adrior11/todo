// TODO: Add a command to toggle config options
use clap::{Parser, Subcommand, Args, ValueEnum};

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
    /// List all todos
    List,

    /// Add a new todo
    Add { 
        /// The description of the todo(s), separated by '::' for multiple items
        #[arg(value_name = "TODO_DESCRIPTION", num_args(1..))]
        args: Vec<String> 
    },

    /// Edit an existing todo item 
    Edit {
        /// The ID of the todo to edit
        #[arg(value_name = "TODO_ID")]
        id: usize,

        /// The new description for the todo
        #[arg(value_name = "NEW_DESCRIPTION")]
        description: Vec<String>,
    },
    
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
    Reset,

    /// Backup the todo List
    Backup {
        /// The optional field of the backup action
        #[command(subcommand)]
        name: Option<BackupAction>,
    },

    /// Sort todos
    Sort {
        /// The optional field to sort by
        #[arg(value_name = "SORT_BY")]
        sort_by: Option<SortBy>,
    },
}

/// Enum representing different backup actions
#[derive(Subcommand)]
pub enum BackupAction {
    /// Create a new backup
    Create,

    /// Shows the contents of a backup a backup file
    Show {
        /// The timestamp of the backup file to show its contents
        #[arg(value_name = "TIMESTAMP")]
        timestamp: String,
    },

    /// Delete existing backups
    Delete(DeleteOptions), 

    /// Restore specific todo items from a backup
    Restore {
        /// The timestamp of the backup file to restore from
        #[arg(value_name = "TIMESTAMP")]
        timestamp: String,

        /// The ID of the todo item to restore from the backup
        #[arg(value_name = "TODO_ID")]
        args: Vec<usize> 
    },

    /// List all backups (default action)
    List, 
}


///Struct representing delete options
#[derive(Args)]
pub struct DeleteOptions {
    /// The specific delete option to use 
    #[command(subcommand)]
    pub option: DeleteOption
}


/// Enum representing delete options
#[derive(Subcommand)]
pub enum DeleteOption {
    /// Delete all backups
    All,

    /// Delete a specific backup by timestamp
    Timestamp {
        /// The timestamp of the backup to delete
        #[arg(value_name = "TIMESTAMP")]
        timestamp: String,
    }
}

/// Enum representing sorting criteria for todos
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SortBy {
    /// Sort by ID
    Id,

    /// Sort by creation date
    Date,

    /// Sort by completion status (default action)
    Done,
}

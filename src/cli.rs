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
    Reset,

    /// Backup the todo List
    Backup {
        /// The optional field of the backup action
        #[arg(value_name = "BACKUP_ACTION")]
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
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BackupAction {
    // TODO: Open command to see it's contents
    /// Create a new backup
    Create,
    // TODO: Give option to delete all or just a specific file (Non optional) 
    /// Delete existing backups
    Delete, 
    /// List all backups (default action)
    List, 
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SortBy {
    /// Sort by ID
    Id,
    /// Sort by creation date
    Date,
    /// Sort by completion status (default action)
    Done,
}

use clap::{Parser, Subcommand, Args, ValueEnum};

/// CLI structure to parse command line arguments
#[derive(Parser)]
#[command(author, version, about = "A simple and flexible command-line todo application built with Rust.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub pattern: Option<Pattern>,
}

/// Enum representing the different command patterns
#[derive(Subcommand)]
pub enum Pattern {
    /// List all todos
    #[command(alias = "l")]
    List,

    /// Add a new todo
    #[command(alias = "a")]
    Add { 
        /// The description of the todo(s), separated by '::' for multiple items
        #[arg(value_name = "TODO_DESCRIPTION", num_args(1..))]
        args: Vec<String> 
    },

    /// Edit an existing todo item 
    #[command(alias = "e")]
    Edit {
        /// The ID of the todo to edit
        #[arg(value_name = "TODO_ID")]
        id: usize,

        /// The new description for the todo
        #[arg(value_name = "NEW_DESCRIPTION")]
        description: Vec<String>,
    },

    /// Filters the todo list by the specified query string
    #[command(alias = "f")]
    Filter {
        /// Query string to filter todos by
        #[arg(value_name = "QUERY")]
        query: Vec<String>,
    },
    
    /// Mark a todo as done
    #[command(alias = "d")]
    Done { 
        /// The ID of the todo to mark as done
        #[arg(value_name = "TODO_ID")]
        args: Vec<usize> 
    },

    /// Mark a todo as not done
    #[command(alias = "u")]
    Undone { 
        /// The ID of the todo to mark as not done
        #[arg(value_name = "TODO_ID")]
        args: Vec<usize>
    },

    /// Star a todo
    #[command(alias = "s")]
    Star {
        /// The ID of the todo to star 
        #[arg(value_name = "TODO_ID")]
        args: Vec<usize>
    },

    /// Remove a todo
    #[command(alias = "r")]
    Rm { 
        /// The ID of the todo to emove
        #[arg(value_name = "TODO_ID")]
        args: Vec<usize> 
    },

    /// Reset the todo list
    Reset,

    /// Sort todos
    #[command(alias = "S")]
    Sort {
        /// The optional field to sort by
        #[arg(value_name = "SORT_BY")]
        sort_by: Option<SortBy>,
    },

    /// Backup the todo List
    #[command(alias = "b")]
    Backup {
        /// The optional field of the backup action
        #[command(subcommand)]
        name: Option<BackupAction>,
    },
}

/// Enum representing different backup actions
#[derive(Subcommand)]
pub enum BackupAction {
    /// List all backups (default action)
    #[command(alias = "l")]
    List, 

    /// Create a new backup
    #[command(alias = "c")]
    Create,

    /// Shows the contents of a backup a backup file
     #[command(alias = "o")]
    Open {
        /// The timestamp of the backup file to show its contents
        #[arg(value_name = "TIMESTAMP")]
        timestamp: String,
    },

    /// Restore specific todo items from a backup
    #[command(alias = "R")]
    Restore {
        /// The timestamp of the backup file to restore from
        #[arg(value_name = "TIMESTAMP")]
        timestamp: String,

        /// The ID of the todo item to restore from the backup
        #[arg(value_name = "TODO_ID")]
        args: Vec<usize> 
    },

    /// Delete existing backups
    #[command(alias = "D")]
    Delete(DeleteOptions), 
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
    #[command(alias = "A")]
    All,

    /// Delete a specific backup by timestamp
    #[command(alias = "t")]
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

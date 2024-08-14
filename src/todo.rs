use serde::{Deserialize, Serialize};
use colored::*;
use anyhow::{anyhow, Context, Result};
use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use chrono::{DateTime, Utc};
use crate::cli::{BackupAction, DeleteOptions, DeleteOption, Pattern, SortBy};
use crate::utils::*;
use crate::config::{Config, load_config_from_lua};

/// Struct representing a Todo item
#[derive(Serialize, Deserialize)]
struct Todo {
    id: usize,
    desc: String,
    done: bool,
    created_at: DateTime<Utc>,
}

/// Struct representing a list of Todo items
#[derive(Serialize, Deserialize, Default)]
pub struct TodoList {
    todos: Vec<Todo>,
    available_ids: BTreeSet<usize>,
    #[serde(skip)]
    config: Config,
}

impl TodoList {
    /// Handle CLI commands
    pub fn handle_cli(&mut self, pattern: Pattern) {
        match pattern {
            Pattern::Add { args } => self.add(args),
            Pattern::Edit { id, description } => self.edit(id, description)
                .unwrap_or_else(|err| eprint!("Error: {}", err)),
            Pattern::List => self.list(),
            Pattern::Done { args } => self.done(args)
                .unwrap_or_else(|err| eprintln!("Error: {}", err)),
            Pattern::Rm { args } => self.rm(args)
                .unwrap_or_else(|err| eprintln!("Error: {}", err)),
            Pattern::Reset => self.reset(),
            Pattern::Sort { sort_by } => self.sort(sort_by),
            Pattern::Backup { name } => self.handle_backup(name),
        }
    }

    /// List all todo items
    // TODO: ✔ ✘
    pub fn list(&self) {
        let max_indent_count = self.todos.iter()
            .map(|todo| todo.id)
            .max()
            .unwrap_or(0)
            .to_string()
            .len();

        for todo in self.todos.iter() {
            let indent = " ".repeat(max_indent_count - todo.id.to_string().len());
            let id_bold = todo.id.to_string().bold();
            if todo.done { 
                println!("{}{} {}", id_bold, indent, todo.desc.strikethrough().dimmed()); 
            } else {
                println!("{}{} {}", id_bold, indent, todo.desc);
            }
        }
    }

    /// Add new todo items
    fn add(&mut self, args: Vec<String>) {
        let arg = args.join(" ");
        
        // Split the input string by double colon to get individual todo items
        let items = arg.split("::");
        for item in items {
            // Trim any extra whitespace from the item description
            let item_desc = item.trim();
            if item_desc.is_empty() {
                continue;
            }

            // Get the smallest available ID or create a new one
            let id = self.get_next_available_id();

            self.todos.push(Todo {
                id,
                desc: item_desc.to_string(),
                done: false,
                created_at: Utc::now(),
            });
        };
        self.list();
    }

    /// Edit the description of an existing todo item 
    fn edit(&mut self, id: usize, description: Vec<String>) -> Result<()> {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
            todo.desc = description.join(" ");
            self.list();
            Ok(())
        } else {
            Err(anyhow!("ID {} not found", id))
        }
    }

    /// Mark todo items as done
    fn done(&mut self, ids: Vec<usize>) -> Result<()> {
        for id in ids {
            if let Some(todo) = self.todos.iter().position(|todo| todo.id == id) {
                self.todos[todo].done = true;
            } else {
                return Err(anyhow!("ID {} not found", id));
            }
        };
        self.list();
        Ok(())
    }

    /// Remove todo items by ID
    fn rm(&mut self, ids: Vec<usize>) -> Result<()>{
        for id in ids {
            if let Some(todo) = self.todos.iter().position(|todo| todo.id == id) {
                self.available_ids.insert(self.todos.remove(todo).id);
            } else {
                return Err(anyhow!("ID {} not found", id));
            }
        };
        self.list();
        Ok(())
    }

    /// Reset the todo list and create a backup file unless statet
    fn reset(&mut self) {
        if self.config.backup_on_reset {
            if let Err(e) = backup_todo_file() {
                eprintln!("Backup deletion error: {}", e);
            } 
        }

        self.todos.clear();
        self.available_ids.clear();
    }

    /// Sort todo items by their completion status
    fn sort(&mut self, sort_by: Option<SortBy>) {
        match sort_by {
            Some(SortBy::Id) => self.todos.sort_by_key(|todo| todo.id),
            Some(SortBy::Date) => self.todos.sort_by_key(|todo| todo.created_at),
            _ => self.todos.sort_by_key(|todo| todo.done),
        }
        self.list();
    }

    /// Handle backup operations based on the provided action
    fn handle_backup(&mut self, backup_action: Option<BackupAction>) {
        match backup_action {
            Some(BackupAction::Create) => self.create_backup(),
            Some(BackupAction::Delete(delete_option)) => self.delete_backup(delete_option),
            Some(BackupAction::Restore { timestamp, args }) => self.restore_backup(&timestamp, args),
            Some(BackupAction::Show { timestamp }) => self.show_backup(&timestamp),
            _ => self.list_backups(),
        }
    }

    /// Create a new backup
    fn create_backup(&self) {
        if let Err(e) = backup_todo_file() {
            eprintln!("Error creating backup: {}", e);
        }
    }

    /// Delete backups based on the specified option
    fn delete_backup(&self, delete_option: DeleteOptions) {
        match delete_option.option {
            DeleteOption::All => {
                if let Err(e) = delete_backup_files() {
                    eprintln!("Error deleting all backups: {}", e);
                }
            }
            DeleteOption::Timestamp { timestamp } => {
                if let Err(e) = delete_specific_backup_file(&timestamp) {
                    eprintln!("Error deleting backup with timestamp {}: {}", timestamp, e);
                }
            }
        }
    }

    /// Restore todo items from a backup
    fn restore_backup(&mut self, timestamp: &str, ids_to_restore: Vec<usize>) {
        match read_todo_list_from_backup(timestamp) {
            Ok(todo_list) => {
                for id in ids_to_restore {
                    if let Some(todo) = self.restore_single_todo_from_backup(&todo_list, id) {
                        self.todos.push(todo);
                    } else {
                        eprintln!("Error restoring backup item with ID {}", id);
                    }
                }

                self.list();
            }
            Err(e) => eprintln!("Error restoring backup from {}: {:?}", timestamp, e),
        }
    }

    /// Restore a single todo item from a backup list 
    fn restore_single_todo_from_backup(&mut self, todo_list: &TodoList, id: usize) -> Option<Todo> {
        if let Some(backup_todo) = todo_list.todos.iter().find(|todo| todo.id == id) {
            Some(Todo {
                id: self.get_next_available_id(),
                desc: backup_todo.desc.clone(),
                done: backup_todo.done,
                created_at: backup_todo.created_at,
            })
        } else {
            eprintln!("Error: Todo item with ID {} not found in backup", id);
            None
        }
    }

    /// Show the contents of a specific backup
    fn show_backup(&self, timestamp: &str) {
        match read_todo_list_from_backup(timestamp) {
            Ok(todo_list) => todo_list.list(),
            Err(e) => eprintln!("Error showing backup contents of {}: {:?}", timestamp, e),
        }
    }

    /// List all available backups
    fn list_backups(&self) {
        if let Err(e) = list_backup_files() {
            eprintln!("Error listing backups: {}", e);
        }
    }

    /// Load todo list from a file
    pub fn load_from_file(file_path: &Path) -> Result<Self> {
        let mut todo_list = read_todo_list_from_file(file_path)?;

        // Load configuration from Lua file
        todo_list.config = load_config_from_lua().context("Failed to load configuration from Lua")?;

        Ok(todo_list)
    }

    /// Save todo list to a file
    pub fn save_to_file(&self, file_path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(&self)
            .context("Failed to serialize todo list")?;

        let mut file = File::create(file_path)
            .context("Failed to create todo file")?;

        file.write_all(content.as_bytes())
            .context("Failed to write todo file")?;

        Ok(())
    }

    /// Retrieve the next available ID or create a new one
    fn get_next_available_id(&mut self) -> usize {
        let next_id = self.available_ids.iter()
            .next()
            .cloned()
            .unwrap_or_else(|| self.todos.len() + 1);

        self.available_ids.remove(&next_id);
        next_id
    } 
}

/// Helper function to read and parse a `TodoList` from a file.
///
/// If the file does not exist, it returns a default `TodoList`.
///
/// # Arguments
///
/// `file_path` - A reference to a `Path` that specifies the location of the file to be read.
///
/// # Returns
///
/// `Result<TodoList>` - On success, returns a `TodoList` parsed from the file contents. 
///   On failure, returns an error indicating the reason for the failure.
///
/// # Errors
///
/// This function will return an error if:
/// 
/// * The file cannot be opened.
/// * The file contents cannot be read.
/// * The contents cannot be parsed as a `TodoList` due to JSON format issues.
fn read_todo_list_from_file(file_path: &Path) -> Result<TodoList> {
    if !file_path.exists() {
        return Ok(TodoList::default());
    }

    let mut file = File::open(file_path).context("Failed to open todo file")?;
    let mut content = String::new();

    file.read_to_string(&mut content).context("Failed to read todo file")?;

    let todo_list: TodoList = serde_json::from_str(&content)
        .context("Failed to parse todo JSON")?;

    Ok(todo_list)
}

/// Helper function to read and parse a `TodoList` from a backup file identified by a timestamp.
///
/// This function constructs the backup file path using the provided timestamp and then calls 
/// `read_todo_list_from_file` to read and parse the `TodoList` from that file.
///
/// # Arguments
///
/// `timestamp` - A string slice that represents the timestamp of the backup file to be read.
///
/// # Returns
///
/// `Result<TodoList>` - On success, returns a `TodoList` parsed from the backup file. 
///   On failure, returns an error indicating the reason for the failure.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The backup file path cannot be constructed.
/// * The backup file cannot be read or parsed, as described in the documentation for 
///   `read_todo_list_from_file`.
fn read_todo_list_from_backup(timestamp: &str) -> Result<TodoList> {
    let backup_path = get_backup_file_path(timestamp)?;
    read_todo_list_from_file(&backup_path)
}

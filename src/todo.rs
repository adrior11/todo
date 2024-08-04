use serde::{Deserialize, Serialize};
use colored::*;
use anyhow::{anyhow, Context, Result};
use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use chrono::{DateTime, Utc};
use crate::cli::{BackupAction, DeleteOption, Pattern, SortBy};
use crate::utils::*;

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
}

impl TodoList {
    /// Handle CLI commands
    pub fn handle_cli(&mut self, pattern: Pattern) {
        match pattern {
            Pattern::Add { args } => self.add(args),
            Pattern::List => self.list(),
            Pattern::Done { args } => self.done(args).unwrap_or_else(|err| eprintln!("Error: {}", err)),
            Pattern::Rm { args } => self.rm(args).unwrap_or_else(|err| eprintln!("Error: {}", err)),
            Pattern::Reset => self.reset(),
            Pattern::Backup { name } => self.backup(name),
            Pattern::Sort { sort_by } => self.sort(sort_by),
        }
    }

    /// List all todo items
    pub fn list(&self) {
        for todo in self.todos.iter() {
            let id_bold = todo.id.to_string().bold();
            if todo.done { 
                println!("{} {}", id_bold, todo.desc.strikethrough().dimmed()); 
            } else {
                println!("{} {}", id_bold, todo.desc);
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
            let id = self.available_ids.iter().next().cloned().unwrap_or_else(|| self.todos.len() + 1);
            self.available_ids.remove(&id);

            self.todos.push(Todo {
                id,
                desc: item_desc.to_string(),
                done: false,
                created_at: Utc::now(),
            });
        };
        self.list();
    }

    /// Mark todo items as done
    fn done(&mut self, ids: Vec<usize>) -> Result<()> {
        for id in ids {
            if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
                self.todos[pos].done = true;
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
            if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
                self.available_ids.insert(self.todos.remove(pos).id);
            } else {
                return Err(anyhow!("ID {} not found", id));
            }
        };
        self.list();
        Ok(())
    }

    /// Reset the todo list and create a backup file unless statet
    fn reset(&mut self) {
        // TODO: Option to disable it per default
        if let Err(e) = backup_todo_file() {
            eprintln!("Backup deletion error: {}", e);
        } 

        self.todos.clear();
        self.available_ids.clear();
    }

    /// Handle backup operations based on the provided action
    fn backup(&self, backup_action: Option<BackupAction>) {
        match backup_action {
            Some(BackupAction::Create) => {
                if let Err(e) = backup_todo_file() {
                    eprintln!("Error creating backup: {}", e);
                }
            },
            Some(BackupAction::Delete(delete_option)) => {
                match delete_option.option {
                    DeleteOption::All => {
                        if let Err(e) = delete_backup_files() {
                            eprintln!("Error deleting backups: {}", e);
                        }
                    }
                    DeleteOption::Timestamp { timestamp } => {
                        if let Err(e) = delete_specific_backup_file(&timestamp) {
                            eprint!("Error deleting backup with timestamp {}: {}",
                                timestamp, e);
                        }
                    }
                }
            },
            _ => {
                if let Err(e) = list_backup_files() {
                    eprintln!("Error listing backups: {}", e);
                }
            }
        }
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

    /// Load todo list from a file
    pub fn load_from_file(file_path: &Path) -> Result<Self> {
        if !file_path.exists() {
            return Ok(TodoList::default());
        }

        let mut file = File::open(file_path).context("Failed to open todo file")?;
        let mut content = String::new();
        file.read_to_string(&mut content).context("Failed to read todo file")?;

        let todo_list: TodoList = serde_json::from_str(&content).context("Failed to parse todo JSON")?;
        Ok(todo_list)
    }

    /// Save todo list to a file
    pub fn save_to_file(&self, file_path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(&self).context("Failed to serialize todo list")?;
        let mut file = File::create(file_path).context("Failed to create todo file")?;
        file.write_all(content.as_bytes()).context("Failed to write todo file")?;
        Ok(())
    }
}


use serde::{Deserialize, Serialize};
use colored::*;
use anyhow::{anyhow, Context, Result};
use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use crate::cli::Pattern;

/// Struct representing a Todo item
#[derive(Serialize, Deserialize)]
struct Todo {
    id: usize,
    desc: String,
    done: bool,
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
            Pattern::Reset => self.reset(),
            Pattern::Rm { args } => self.rm(args).unwrap_or_else(|err| eprintln!("Error: {}", err)),
            Pattern::Sort => self.sort(),
        }
    }

    /// Add new todo items
    fn add(&mut self, items: Vec<String>) {
        items.iter().for_each(|item| {
            // Get the smallest available ID or create a new one
            let id = self.available_ids.iter().next().cloned().unwrap_or_else(|| self.todos.len() + 1);
            self.available_ids.remove(&id);

            self.todos.push(Todo {
                id,
                desc: item.to_string(),
                done: false,
            });
        })
    }


    // TODO: Maybe adjust the visualisation of todos using [ ] & [*]
    /// List all todo items
    pub fn list(&self) {
        for todo in self.todos.iter() {
            let id_bold = todo.id.to_string().bold();
            if todo.done { 
                println!("{} {}", id_bold, todo.desc.strikethrough()); 
            } else {
                println!("{} {}", id_bold, todo.desc);
            }
        }
    }

    /// Mark todo items as done
    fn done(&mut self, ids: Vec<usize>) -> Result<()> {
        for id in ids {
            if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
                self.available_ids.insert(self.todos.remove(pos).id);
            } else {
                return Err(anyhow!("ID {} not found", id));
            }
        }
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
        } 
        Ok(())
    }

    /// Reset the todo list
    fn reset(&mut self) {
        self.todos.clear();
        self.available_ids.clear();
    }

    /// Sort todo items by their completion status
    fn sort(&mut self) {
        self.todos.sort_by_key(|todo| todo.done);
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

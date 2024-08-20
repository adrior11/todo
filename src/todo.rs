// TODO: Adjust all the self.list() calls with messages instead
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Context, Result};
use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use chrono::{DateTime, Utc};
use crate::cli::{BackupAction, DeleteOptions, DeleteOption, Pattern, SortBy};
use crate::render::render_todo_list;
use crate::utils::*;
use crate::config::{Config, load_config_from_lua};
// use crate::modify_todos;

// TODO: Implement boards  

/// Struct representing a Todo item
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub(crate) id: usize,
    pub(crate) desc: String,
    pub(crate) is_complete: bool,
    pub(crate) is_starred: bool,
    pub(crate) timestamp: DateTime<Utc>,
    // TODO: Tags 
    // TODO: Notes 
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
    pub fn handle_cli(&mut self, pattern: Pattern) -> Result<()> {
        match pattern {
            Pattern::List => self.list(),
            Pattern::Add { args } => self.add(args),
            Pattern::Edit { id, description } => self.edit(id, description)?,
            Pattern::Filter { query } => self.filter(query)?,
            Pattern::Done { args } => self.done(args)?,
            Pattern::Undone { args } => self.undone(args)?,
            Pattern::Star { args } => self.star(args)?,
            Pattern::Rm { args } => self.rm(args)?,
            Pattern::Reset => self.reset()?,
            Pattern::Sort { sort_by } => self.sort(sort_by),
            Pattern::Backup { name } => self.handle_backup(name)?,
        }
        Ok(())
    }

    /// List all todo items
    pub fn list(&self) {
        let todos_refs: Vec<&Todo> = self.todos.iter().collect();
        render_todo_list(&todos_refs, &self.config)
    }

    /// Add new todo items
    fn add(&mut self, args: Vec<String>) {
        // Join arguments into a single string and split by "::" to handle multiple todo items
        let items: Vec<String> = args.join(" ")
            .split("::")
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect();

        for item_desc in items {
            let id = self.get_next_available_id();
            self.todos.push(Todo {
                id,
                desc: item_desc,
                is_complete: false,
                is_starred: false,
                timestamp: Utc::now(),
            });
        }

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

    /// Filters the todo list based on a query string.
    // TODO: Allow filtering for @Board; #Tag; Dates (regex?)
    fn filter(&self, query: Vec<String>) -> Result<()> {
        // Join the query list into a single string and split by "::" to handle multi-query
        let queries: Vec<String> = query.join(" ")
            .to_lowercase()
            .split("::")
            .map(|q| q.trim().to_string())
            .collect();

        // Filter the todo list based on the queries
        let filtered_todos: Vec<&Todo> = self.todos.iter()
            .filter(|todo| {
                let desc = todo.desc.to_lowercase();
                queries.iter().any(|q| desc.contains(q))
            })
            .collect();

        if filtered_todos.is_empty() {
            println!("No results found for query: {:?}", queries);
            return Ok(());
        }

        render_todo_list(filtered_todos.as_slice(), &self.config);
        Ok(())
    }

    /// Mark todo items as done
    fn done(&mut self, ids: Vec<usize>) -> Result<()> {
        for id in ids {
            if let Some(todo) = self.todos.iter().position(|todo| todo.id == id) {
                self.todos[todo].is_complete = true;
            } else {
                return Err(anyhow!("ID {} not found", id));
            }
        };
        self.list();
        Ok(())
    }

    /// Mark todo items as not done 
    fn undone(&mut self, ids: Vec<usize>) -> Result<()> {
        for id in ids {
            if let Some(todo) = self.todos.iter().position(|todo| todo.id == id) {
                self.todos[todo].is_complete = false;
            } else {
                return Err(anyhow!("ID {} not found", id));
            }
        };
        self.list();
        Ok(())
    }

    /// Mark todo items as star 
    fn star(&mut self, ids: Vec<usize>) -> Result<()> {
        for id in ids {
            if let Some(todo) = self.todos.iter().position(|todo| todo.id == id) {
                toggle_bool!(self.todos[todo].is_starred);
            } else {
                return Err(anyhow!("ID {} not found", id));
            }
        };
        self.list();
        Ok(())
    }

    /// Remove todo items by ID
    fn rm(&mut self, ids: Vec<usize>) -> Result<()>{
        // FIX: MACRO
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
    fn reset(&mut self) -> Result<()> {
        if self.config.backup_on_reset {
            backup_todo_file().context("Backup deletion error")?;
        }

        self.todos.clear();
        self.available_ids.clear();

        Ok(())
    }

    /// Sort todo items by their completion status
    fn sort(&mut self, sort_by: Option<SortBy>) {
        match sort_by {
            Some(SortBy::Id) => self.todos.sort_by_key(|todo| todo.id),
            Some(SortBy::Date) => self.todos.sort_by_key(|todo| todo.timestamp),
            _ => self.todos.sort_by_key(|todo| todo.is_complete),
        }
        self.list();
    }

    /// Handle backup operations based on the provided action
    fn handle_backup(&mut self, backup_action: Option<BackupAction>) -> Result<()> {
        match backup_action {
            Some(BackupAction::Create) => self.create_backup()?,
            Some(BackupAction::Delete(delete_option)) => self.delete_backup(delete_option)?,
            Some(BackupAction::Restore { timestamp, args }) => self.restore_backup(&timestamp, args)?,
            Some(BackupAction::Open { timestamp }) => self.show_backup(&timestamp)?,
            _ => self.list_backups()?,
        }
        Ok(())
    }

    /// Create a new backup
    fn create_backup(&self) -> Result<()> {
        backup_todo_file().context("Error creating backup")?;
        Ok(())
    }

    /// Delete backups based on the specified option
    fn delete_backup(&self, delete_option: DeleteOptions) -> Result<()> {
        match delete_option.option {
            DeleteOption::All => delete_backup_files().context("Error deleting all backups")?,
            DeleteOption::Timestamp { timestamp } => delete_specific_backup_file(&timestamp).context(format!("Error deleting backup with timestamp {}", timestamp))?,
        }
        Ok(())
    }

    /// Restore todo items from a backup
    fn restore_backup(&mut self, timestamp: &str, ids_to_restore: Vec<usize>) -> Result<()> {
        let todo_list = read_todo_list_from_backup(timestamp)
            .context(format!("Error restoring backup from {}. The item may not exist in the specified backup.", timestamp))?;

        for id in ids_to_restore {
            let todo = self.restore_single_todo_from_backup(&todo_list, id)
                .context(format!("Error restoring backup item with ID {}", id))?;
            self.todos.push(todo);
        }

        Ok(())
    }

    /// Restore a single todo item from a backup list 
    fn restore_single_todo_from_backup(&mut self, todo_list: &TodoList, id: usize) -> Option<Todo> {
        if let Some(backup_todo) = todo_list.todos.iter().find(|todo| todo.id == id) {
            Some(Todo {
                id: self.get_next_available_id(),
                desc: backup_todo.desc.clone(),
                is_complete: backup_todo.is_complete,
                is_starred: backup_todo.is_starred,
                timestamp: backup_todo.timestamp,
            })
        } else {
            eprintln!("Error: Todo item with ID {} not found in backup", id);
            None
        }
    }

    /// Show the contents of a specific backup
    fn show_backup(&self, timestamp: &str) -> Result<()> {
        let todo_list = read_todo_list_from_backup(timestamp)
            .context(format!("Error showing reading contents of {}", timestamp))?;
        todo_list.list();
        Ok(())
    }

    /// List all available backups
    fn list_backups(&self) -> Result<()> {
        list_backup_files().context("Error listing backups")?;
        Ok(())
    }

    /// Load todo list from a file
    pub fn load_from_file(file_path: &Path) -> Result<Self> {
        let mut todo_list = read_todo_list_from_file(file_path)?;

        // Load configuration from Lua file
        todo_list.config = load_config_from_lua()
            .context("Failed to load configuration from Lua")?;

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
        if let Some(&next_id) = self.available_ids.iter().next() {
            self.available_ids.remove(&next_id);
            next_id
        } else {
            self.todos.len() + 1
        }
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

#[cfg(test)]
mod tests {
    use crate::todo::TodoList;
    use std::collections::BTreeSet;
    use std::path::Path;

    // Helper function to create a TodoList with predefined todos
    fn create_todo_list_with_items() -> TodoList {
        let mut todo_list = TodoList::default();
        todo_list.add(vec!["First task".to_string()]);
        todo_list.add(vec!["Second task".to_string()]);
        todo_list
    }

    #[test]
    fn test_add_todos_generates_unique_ids() {
        let mut todo_list = TodoList::default();
        todo_list.add(vec!["First task".to_string()]);
        todo_list.add(vec!["Second task".to_string()]);
        todo_list.add(vec!["Third task".to_string()]);
        
        assert_eq!(todo_list.todos.len(), 3);
        let ids: BTreeSet<_> = todo_list.todos.iter().map(|todo| todo.id).collect();
        assert_eq!(ids.len(), 3); // All IDs should be unique
    }

    #[test]
    fn test_todo_list_reset() {
        let mut todo_list = create_todo_list_with_items();
        assert_eq!(todo_list.todos.len(), 2);
        
        let res = todo_list.reset();
        assert!(res.is_ok());
        assert_eq!(todo_list.todos.len(), 0);
        assert_eq!(todo_list.available_ids.len(), 0);
    }

    #[test]
    fn test_mark_todos_as_done() {
        let mut todo_list = create_todo_list_with_items();
        let ids: Vec<usize> = todo_list.todos.iter().map(|todo| todo.id).collect();
        
        todo_list.done(ids.clone()).expect("Failed to mark todos as done");

        for todo in todo_list.todos {
            assert!(todo.is_complete, "Todo item with ID {} was not marked as done", todo.id);
        }
    }

    #[test]
    fn test_mark_todos_as_star() {
        let mut todo_list = create_todo_list_with_items();
        let ids: Vec<usize> = todo_list.todos.iter().map(|todo| todo.id).collect();

        todo_list.star(ids.clone()).expect("Failed to mark todos as star");

        for todo in todo_list.todos {
            assert!(todo.is_starred, "Todo item with ID {} was not marked as star", todo.id);
        }
    }

    #[test]
    fn test_remove_todo() {
        let mut todo_list = create_todo_list_with_items();
        let id_to_remove = todo_list.todos[0].id;

        todo_list.rm(vec![id_to_remove]).expect("Failed to remove todo");

        assert!(todo_list.todos.iter().all(|todo| todo.id != id_to_remove), "Todo with ID {} was not removed", id_to_remove);
    }

    #[test]
    fn test_edit_todo() {
        let mut todo_list = create_todo_list_with_items();
        let id_to_edit = todo_list.todos[0].id;
        let new_desc = vec!["Updated task description".to_string()];

        todo_list.edit(id_to_edit, new_desc.clone()).expect("Failed to edit todo");

        assert_eq!(todo_list.todos[0].desc, new_desc.join(" "));
    }

    #[test]
    fn test_remove_nonexistent_todo() {
        let mut todo_list = create_todo_list_with_items();
        let non_existent_id = 999;

        let result = todo_list.rm(vec![non_existent_id]);
        assert!(result.is_err(), "Removing non-existent todo should fail");
    }

    #[test]
    fn test_edit_nonexistent_todo() {
        let mut todo_list = create_todo_list_with_items();
        let non_existent_id = 999;
        let new_desc = vec!["Non-existent task".to_string()];

        let result = todo_list.edit(non_existent_id, new_desc);
        assert!(result.is_err(), "Editing non-existent todo should fail");
    }

    #[test]
    fn test_save_and_load_todo_list() {
        let file_path = Path::new("test_todos.json");
        let todo_list = create_todo_list_with_items();

        // Save the list to a file
        todo_list.save_to_file(file_path).expect("Failed to save todo list");

        // Load the list from the file
        let loaded_todo_list = TodoList::load_from_file(file_path).expect("Failed to load todo list");

        assert_eq!(loaded_todo_list.todos.len(), todo_list.todos.len());

        // Clean up test file
        std::fs::remove_file(file_path).expect("Failed to delete test file");
    }

    #[test]
    fn test_todo_id_regeneration_after_removal() {
        let mut todo_list = TodoList::default();

        // Add some todos
        todo_list.add(vec!["First task".to_string()]);
        todo_list.add(vec!["Second task".to_string()]);

        // Remove the first todo
        let id_to_remove = todo_list.todos[0].id;
        todo_list.rm(vec![id_to_remove]).expect("Failed to remove todo");

        // Add a new todo, which should reuse the removed ID
        todo_list.add(vec!["Third task".to_string()]);
        
        assert_eq!(todo_list.todos.len(), 2);
        assert!(todo_list.todos.iter().any(|todo| todo.id == id_to_remove), "ID was not reused");
    }
}

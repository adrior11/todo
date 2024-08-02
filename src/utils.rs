use anyhow::{anyhow, Context, Ok, Result};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Get the path to the todo file
pub fn get_todo_file_path() -> Result<PathBuf> {
    let mut path = dirs::data_local_dir().ok_or_else(|| anyhow::anyhow!("Home directory not found"))?;
    path.push("todo_app");
    fs::create_dir_all(&path).context("Failed to create todo_app directory")?;
    path.push("todos.json");
    Ok(path)
}

/// Get the path to the latest backup file, if it exists
pub fn delete_backup_file() -> Result<()> {
    let todo_path = get_todo_file_path()?;
    let backup_dir = todo_path.parent().unwrap();

    for entry in fs::read_dir(backup_dir).context("Failed to read backup directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().unwrap().to_str().unwrap().starts_with("todos_backup_") {
            let _ = fs::remove_file(&path).with_context(|| format!("Failed to remove backup file: {:?}", path));
        }
    }

    Ok(())
}

/// Backup the existing todo file
pub fn backup_todo_file() -> Result<PathBuf> {
    let todo_path = get_todo_file_path()?;
    if todo_path.exists() {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let backup_path = todo_path.with_file_name(format!("todos_backup_{}.json", timestamp));

        // Delete any existing backup files
        delete_backup_file()?;

        fs::rename(&todo_path, &backup_path)
            .with_context(|| format!("Failed to rename todo file to backup path: {:?}", backup_path))?;
        Ok(backup_path)
    } else {
        Err(anyhow!("Todo file does not exist"))
    }
}

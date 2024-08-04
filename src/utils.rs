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

/// Get the path to the backup directory
fn get_backup_dir_path() -> Result<PathBuf> {
    let mut path = dirs::data_local_dir().ok_or_else(|| anyhow::anyhow!("Home directory not found"))?;
    path.push("todo_app");
    path.push("backup");
    fs::create_dir_all(&path).context("Failed to create backup directory")?;
    Ok(path)
}

/// Get a backup file given by a specific timestamp
pub fn get_backup_file_path(timestamp: &str) -> Result<PathBuf> {
    let backup_dir = get_backup_dir_path()?;
    let backup_file = backup_dir.join(format!("todos_backup_{}.json", timestamp));

    if backup_file.exists() {
        Ok(backup_file)
    } else {
        Err(anyhow!("Backup file with timestamp {} does not exist", timestamp))
    }
}

/// Delete all existing backup files
pub fn delete_backup_files() -> Result<()> {
    let backup_dir = get_backup_dir_path()?;

    for entry in fs::read_dir(backup_dir).context("Failed to read backup directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().unwrap().to_str().unwrap().starts_with("todos_backup_") {
            let _ = fs::remove_file(&path).with_context(|| format!("Failed to remove backup file: {:?}", path));
        }
    }

    Ok(())
}

/// Deletes a specific backup file given by its timestamp
pub fn delete_specific_backup_file(timestamp: &str) -> Result<()> {
    let backup_file = get_backup_file_path(timestamp)?;
    fs::remove_file(&backup_file).with_context(|| format!("Failed to remove backup file: {:?}", backup_file))?;
    Ok(())
}

/// Backup the current todo file
pub fn backup_todo_file() -> Result<PathBuf> {
    let todo_path = get_todo_file_path()?;
    let backup_dir = get_backup_dir_path()?;


    if todo_path.exists() {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let backup_path = backup_dir.join(format!("todos_backup_{}.json", timestamp));

        fs::copy(&todo_path, &backup_path)
            .with_context(|| format!("Failed to rename todo file to backup path: {:?}", backup_path))?;
        Ok(backup_path)
    } else {
        Err(anyhow!("Todo file does not exist"))
    }
}

/// List all backup files and print their timestamps
pub fn list_backup_files() -> Result<()> {
    let backup_dir = get_backup_dir_path()?;

    for entry in fs::read_dir(backup_dir).context("Failed to read backup directory")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            if file_name.starts_with("todos_backup_") {
                if let Some(timestamp) = trim_backup_file_name(file_name) {
                    println!("{}", timestamp);
                } else {
                    eprint!("The backup file name format is incorrect")
                }
            }
        }
    }

    Ok(())
}

/// Trim the beginning and end of the backup file name to extract the timestamp 
fn trim_backup_file_name(input: &str) -> Option<&str> {
    if let Some(stripped) = input.strip_prefix("todos_backup_") {
        return stripped.strip_suffix(".json");
    }
    None
}

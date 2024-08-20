use anyhow::{anyhow, Context, Ok, Result};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Get the path to the todo file.
///
/// If the directory does not exist, it attempts to create it.
///
/// # Returns
///
/// `Result<PathBuf>` - The full path to the `todos.json` file, or an error if the path could not be determined or created.
pub fn get_todo_file_path() -> Result<PathBuf> {
    let mut path = get_app_dir().context("Could not locate or create the application data directory for storing todo files")?;
    path.push("todos.json");
    Ok(path)
}

/// Get the path to the backup directory.
///
/// If the directory does not exist, it attempts to create it.
///
/// # Returns
///
/// `Result<PathBuf>` - The full path to the backup directory, or an error if the directory could not be created.
fn get_backup_dir_path() -> Result<PathBuf> {
    let mut path = get_app_dir()?;
    path.push("backup");
    fs::create_dir_all(&path).context("Failed to create backup directory")?;
    Ok(path)
}

/// Get the path to the application data directory.
///
/// If the directory does not exist, it attempts to create it.
///
/// # Returns
///
/// `Result<PathBuf>` - The full path to the application's data directory, or an error if the directory could not be determined or created.
fn get_app_dir() -> Result<PathBuf> {
    let mut path = dirs::data_local_dir().ok_or_else(|| anyhow!("Local data directory not found"))?;
    path.push("todo_app");
    fs::create_dir_all(&path).context("Failed to create todo_app directory")?;
    Ok(path)
}

/// Get the path to the application configuration directory.
///
/// If the directory does not exist, it attempts to create it.
///
/// # Returns
///
/// `Result<PathBuf>` - The full path to the application's configuration directory, or an error if the directory could not be determined or created.
pub fn get_config_dir() -> Result<PathBuf> {
    let mut path = dirs::config_dir().ok_or_else(|| anyhow!("Configuration directory not foudn"))?;
    path.push("todo_app");
    fs::create_dir_all(&path).context("Failed to create configuration directory")?;
    Ok(path)
}

/// Get the path to a backup file based on a specific timestamp.
///
/// It verifies if the backup file exists and returns the path if found, otherwise returns an error.
///
/// # Arguments
///
/// `timestamp` - A string slice representing the timestamp of the desired backup file.
///
/// # Returns
///
/// `Result<PathBuf>` - The full path to the backup file, or an error if the file does not exist.
pub fn get_backup_file_path(timestamp: &str) -> Result<PathBuf> {
    let backup_file = get_backup_dir_path()?.join(format!("todos_backup_{}.json", timestamp));

    if backup_file.exists() {
        Ok(backup_file)
    } else {
        Err(anyhow!("Backup file with timestamp {} does not exist", timestamp))
    }
}

/// Get the path to the configuration file.
///
/// This function returns the file path for the `config.lua` file within the application's configuration directory.
/// If the directory does not exist, it attempts to create it.
///
/// # Returns
///
/// `Result<PathBuf>` - The full path to the `config.lua` file, or an error if the path could not be determined or created.
pub fn get_config_file_path() -> Result<PathBuf> {
    let mut path = get_config_dir()?;
    path.push("config.lua");
    Ok(path)
}


/// Delete all existing backup files.
///
/// This function deletes all backup files in the backup directory that follow the naming convention `todos_backup_*.json`.
/// It skips any files that do not match this pattern.
///
/// # Returns
///
/// `Result<()>` - Returns `Ok(())` if all matching backup files are successfully deleted, or an error if the directory cannot be read.
pub fn delete_backup_files() -> Result<()> {
    let backup_dir = get_backup_dir_path()?;

    for entry in fs::read_dir(backup_dir).context("Failed to read backup directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().unwrap().to_str().unwrap().starts_with("todos_backup_") {
            fs::remove_file(&path).with_context(|| format!("Failed to remove backup file: {:?}", path))?;
        }
    }

    Ok(())
}

/// Delete a specific backup file given by its timestamp.
///
/// It returns an error if the backup file does not exist or cannot be deleted.
///
/// # Arguments
///
/// `timestamp` - A string slice representing the timestamp of the backup file to delete.
///
/// # Returns
///
/// `Result<()>` - Returns `Ok(())` if the backup file is successfully deleted, or an error if it cannot be found or deleted.
pub fn delete_specific_backup_file(timestamp: &str) -> Result<()> {
    let backup_file = get_backup_file_path(timestamp)?;
    fs::remove_file(&backup_file).with_context(|| format!("Failed to delete backup file {} at: {:?}. Please check if the file exists.", timestamp, backup_file))?;
    Ok(())
}

/// Backup the current todo file.
///
/// This function creates a backup of the current `todos.json` file by copying it to the backup directory with a timestamped filename. 
/// It returns the path to the created backup file.
///
/// # Returns
///
/// `Result<PathBuf>` - The full path to the newly created backup file, or an error if the backup could not be completed.
pub fn backup_todo_file() -> Result<PathBuf> {
    let todo_path = get_todo_file_path()?;
    if !todo_path.exists() {
        return Err(anyhow!("Todo file does not exist. Please ensure that the todo list has been created before attempting to back it up."));
    }

    let backup_dir = get_backup_dir_path()?;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let backup_path = backup_dir.join(format!("todos_backup_{}.json", timestamp));

    fs::copy(&todo_path, &backup_path)
        .with_context(|| format!("Failed to rename todo file to backup path: {:?}", backup_path))?;

    Ok(backup_path)
}

/// List all backup files and print their timestamps.
///
/// This function lists all the backup files in the backup directory by extracting and printing the timestamps from their filenames. 
/// It skips files that do not match the expected naming convention.
///
/// # Returns
///
/// `Result<()>` - Returns `Ok(())` if the files are successfully listed, or an error if the directory cannot be read.
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

/// Trim the prefix and suffix of a backup file name to extract the timestamp.
///
/// This helper function removes the `todos_backup_` prefix and `.json` suffix from a backup file name to return just the timestamp.
///
/// # Arguments
///
/// `input` - The full backup file name as a string slice.
///
/// # Returns
///
/// `Option<&str>` - Returns the extracted timestamp as a string slice, or `None` if the input does not match the expected format.
fn trim_backup_file_name(input: &str) -> Option<&str> {
    input.strip_prefix("todos_backup_").and_then(|s| s.strip_suffix(".json")) 
}

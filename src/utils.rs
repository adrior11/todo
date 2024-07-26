use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Get the path to the todo file
pub fn get_todo_file_path() -> Result<PathBuf> {
    let mut path = dirs::data_local_dir().ok_or_else(|| anyhow::anyhow!("Home directory not found"))?;
    path.push("todo_app");
    fs::create_dir_all(&path)?;
    path.push("todos.json");
    Ok(path)
}

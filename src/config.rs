use rlua::Lua;
use serde::Deserialize;
use anyhow::{Context, Result};
use std::fs;
use crate::utils::get_config_file_path;

/// Struct representing the configuration settings
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Determines if a backup should be created on reset
    pub backup_on_reset: bool,
}

impl Default for Config {
    /// Provides the default configuration settings
    fn default() -> Self {
        Config {
            backup_on_reset: true,
        }
    }
}

/// Loads the configuration from a Lua file.
/// 
/// If the configuration file does not exist, it creates one with default values.
/// 
/// # Returns
/// 
/// `Result<Config>` - Returns a `Config` struct populated with the settings from the Lua file, 
/// or an error if the file could not be loaded or parsed.
///
/// # Errors
///
/// This function will return an error if the configuration file cannot be read or parsed.
pub fn load_config_from_lua() -> Result<Config> {
    let config_path = get_config_file_path()?;
    
    // Check if the configuration file exists, if not, create it with default values
    if !config_path.exists() {
        let default_config = Config::default();
        let default_lua = generate_lua_config!(
            "backup_on_reset" => default_config.backup_on_reset,
        );
        fs::write(&config_path, default_lua).context("Failed to write default config.lua")?;
    }

    let lua = Lua::new();
    
    lua.load(&std::fs::read_to_string(&config_path)?).exec()?;
    let globals = lua.globals();
    let config: rlua::Table = globals.get("config")?;
    let backup_on_reset: bool = config.get("backup_on_reset")?;
    
    Ok(Config {
        backup_on_reset,
    })
}

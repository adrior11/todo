use rlua::Lua;
use serde::Deserialize;
use anyhow::{Context, Result};
use std::fs;
use crate::utils::get_config_file_path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub backup_on_reset: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            backup_on_reset: true,
        }
    }
}

pub fn load_config_from_lua() -> Result<Config> {
    let config_path = get_config_file_path()?;
    
    // Check if the configuration file exists, if not, create it with default values
    if !config_path.exists() {
        let default_config = Config::default();
        let default_lua = format!(
            "config = {{ backup_on_reset = {} }}",
            default_config.backup_on_reset
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

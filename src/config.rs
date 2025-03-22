use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: 1200,
            window_height: 800,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        println!("Loading config from: {:?}", config_path);

        let config = if let Ok(contents) = fs::read_to_string(&config_path) {
            println!("Found config file, contents: {}", contents);
            toml::from_str(&contents).unwrap_or_else(|e| {
                println!("Error parsing config: {}", e);
                Self::default()
            })
        } else {
            println!("No config file found, using defaults");
            Self::default()
        };

        println!("Loaded config: {:?}", config);
        config
    }

    pub fn save(&self) {
        let config_path = Self::get_config_path();
        println!("Saving config to: {:?}", config_path);

        if let Some(parent) = config_path.parent() {
            println!("Creating parent directory: {:?}", parent);
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(contents) = toml::to_string_pretty(self) {
            println!("Saving config contents: {}", contents);
            if let Err(e) = fs::write(&config_path, contents) {
                println!("Error saving config: {}", e);
            } else {
                println!("Config saved successfully");
            }
        } else {
            println!("Error converting config to TOML");
        }
    }

    fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("okay-you-very-pro");
        path.push("config.toml");
        println!("Config path: {:?}", path);
        path
    }
}

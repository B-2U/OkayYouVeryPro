use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{error, info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub selected_folder: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: 1200,
            window_height: 800,
            selected_folder: None,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        info!("Loading config from: {:?}", config_path);

        let config = if let Ok(contents) = fs::read_to_string(&config_path) {
            info!("Found config file, contents: {}", contents);
            toml::from_str(&contents).unwrap_or_else(|e| {
                error!("Error parsing config: {}", e);
                Self::default()
            })
        } else {
            warn!("No config file found, using defaults");
            Self::default()
        };

        info!("Loaded config: {:?}", config);
        config
    }

    pub fn save(&self) {
        let config_path = Self::get_config_path();
        info!("Saving config to: {:?}", config_path);

        if let Some(parent) = config_path.parent() {
            info!("Creating parent directory: {:?}", parent);
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(contents) = toml::to_string_pretty(self) {
            info!("Saving config contents: {}", contents);
            if let Err(e) = fs::write(&config_path, contents) {
                error!("Error saving config: {}", e);
            } else {
                info!("Config saved successfully");
            }
        } else {
            error!("Error converting config to TOML");
        }
    }

    fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("okay-you-very-pro");
        path.push("config.toml");
        info!("Config path: {:?}", path);
        path
    }

    pub fn replay_path(&self) -> std::path::PathBuf {
        self.selected_folder
            .as_ref()
            .map(|f| std::path::Path::new(f).join("replays"))
            .unwrap_or_default()
    }
}

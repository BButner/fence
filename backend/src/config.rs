use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::region::Region;

/// Persistent Configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Port
    pub port: u16,
    /// Automatically Lock on Startup
    pub auto_lock: bool,
    /// The currently saved Regions
    pub regions: Vec<Region>,

    loaded_path: PathBuf,
}

/// Implementation of Config
impl Config {
    /// Create a new Config
    pub fn new(loaded_path: PathBuf) -> Self {
        Config {
            port: 50051,
            auto_lock: false,
            regions: Vec::new(),
            loaded_path,
        }
    }

    /// Load the Config from the given Path, else defaults to directories Base Config Directory
    pub async fn load(path: Option<&str>) -> Option<Self> {
        let config_path = directories::BaseDirs::new()
            .unwrap()
            .config_dir()
            .join("fence")
            .join("config.json");

        if path.is_none() && !config_path.exists() {
            let config = Config::new(config_path);
            config.save().await;
            return Some(config);
        }

        if path.is_some() && !PathBuf::from(path.unwrap()).exists() {
            let config = Config::new(PathBuf::from(path.unwrap()));
            config.save().await;
            return Some(config);
        }

        match path {
            Some(path) => Self::load_config(path).await,
            None => Self::load_config(config_path.to_str().unwrap()).await,
        }
    }

    /// Save the Config
    pub async fn save(self: &Self) {
        Self::save_config(self).await;
    }

    /// Load the Config from the given Path, asynchronously
    async fn load_config(path: &str) -> Option<Self> {
        let mut file = tokio::fs::File::open(path).await.unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents).await;

        let loaded = serde_json::from_str::<Config>(&contents);

        match loaded {
            Ok(config) => Some(config),
            Err(_) => None,
        }
    }

    /// Save the Config, asynchronously
    async fn save_config(config: &Self) {
        let serialized = serde_json::to_string_pretty(&config).unwrap();
        let file = tokio::fs::File::create(&config.loaded_path()).await;

        match file {
            Ok(mut file) => {
                let _ = file.write_all(serialized.as_bytes()).await;
            }
            Err(_) => {
                let _ = tokio::fs::create_dir_all(config.loaded_path().parent().unwrap()).await;
                let mut file = tokio::fs::File::create(&config.loaded_path())
                    .await
                    .unwrap();
                let _ = file.write_all(serialized.as_bytes()).await;
            }
        }
    }

    /// Gets the Path the Config was loaded from
    pub fn loaded_path(&self) -> &PathBuf {
        &self.loaded_path
    }
}

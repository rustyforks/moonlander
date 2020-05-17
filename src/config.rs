use crate::DIRS;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = load_config().expect("Cannot load config");
}

#[derive(Serialize, Deserialize)]
pub struct Color(u8, u8, u8);

#[derive(Serialize, Deserialize)]
pub struct ContentTheme {}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub content: ContentTheme,
}

#[derive(Serialize, Deserialize)]
pub struct Window {
    pub size: (u32, u32),

    pub fps: u32,
    pub vsync: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub homepage: String,

    pub window: Window,
    pub theme: Theme,
}

fn default_config() -> Config {
    Config {
        homepage: "gemini://gemini.circumlunar.space".to_owned(),

        window: Window {
            size: (800, 600),

            fps: 60,
            vsync: false, // Causes flickering when resizing
        },

        theme: Theme {
            content: ContentTheme {},
        },
    }
}

fn load_config() -> Result<Config> {
    let path = DIRS.config_dir().join("config.toml");
    if path.exists() {
        let content = fs::read_to_string(&path).context("Cannot read config")?;

        log::info!("Config read from {}", path.to_string_lossy());
        toml::from_str(&content).context("Cannot parse TOML")
    } else {
        let cfg = default_config();
        let out = toml::to_string_pretty(&cfg).context("Cannot serialize TOML")?;

        let parent = path
            .parent()
            .expect("Config path doesn't have a parent directory?");

        fs::create_dir_all(parent).context("Cannot create config parent directories")?;
        fs::write(&path, out).context("Cannot write default config")?;

        log::info!("Written default config to {}", path.to_string_lossy());
        Ok(cfg)
    }
}

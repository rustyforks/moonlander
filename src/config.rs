use crate::DIRS;

use anyhow::{Context, Result};
use relm_moonrender::moonrender::Theme;
use serde::{Deserialize, Serialize};
use std::fs;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = load_config().expect("Cannot load config");
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub homepage: String,

    pub theme: Theme,
}

fn default_config() -> Config {
    Config {
        homepage: "gemini://gemini.circumlunar.space".to_owned(),

        theme: Theme::default(),
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
            .context("Config path doesn't have a parent directory?")?;

        fs::create_dir_all(parent).context("Cannot create config parent directories")?;
        fs::write(&path, out).context("Cannot write default config")?;

        log::info!("Written default config to {}", path.to_string_lossy());
        Ok(cfg)
    }
}

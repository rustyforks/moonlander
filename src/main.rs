#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod gui;

use anyhow::{anyhow, Result};
use directories_next::ProjectDirs;
use log::LevelFilter;
use relm::Widget;

pub use config::CONFIG;

lazy_static::lazy_static! {
    pub static ref DIRS: ProjectDirs = ProjectDirs::from("com", "ecmelberk", "moonlander").expect("Cannot get project directories");
}

fn main() -> Result<()> {
    let mut b = pretty_env_logger::formatted_builder();
    b.filter_level(if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    b.try_init()?;

    log::info!("Hello, moon!");

    gui::Win::run(()).map_err(|_| anyhow!("Cannot run GTK application"))
}

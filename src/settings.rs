use std::fs;
use bevy_ecs::system::Resource;
use serde::{Serialize, Deserialize};

#[derive(Resource, Serialize, Deserialize)]
pub struct Settings {
    pub width: u32,
    pub height: u32,
    pub title: String,
    //TODO: Add fullscreen
    pub swap_interval: i32,
    pub is_wireframe: bool
}

pub const SETTINGS_LOCATION: &str = "resources/settings.toml";

impl Default for Settings {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            title: "Game - Butter Engine".to_string(),
            swap_interval: -1,
            is_wireframe: false
        }
    }
}

pub fn load() -> Settings {
    let file = fs::read_to_string(SETTINGS_LOCATION);
    match file {
        Ok(file_string) => toml::from_str(&file_string).unwrap_or_default(),
        Err(_) => Settings::default(),
    }
}

pub fn save(settings: &Settings) -> Result<&Settings, String> {
    match fs::write(SETTINGS_LOCATION, toml::to_string(&settings).expect("Failed to serialize settings!")) {
        Ok(_) => Ok(settings),
        Err(_) => Err("Failed to save settings!".to_string()),
    }
}
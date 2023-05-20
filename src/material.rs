use std::fs;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Material {
    pub name: String,
    pub textures: Vec<String>,
    pub shader: String,
}

impl Material {
    pub fn new(name: &str) -> Material {
        let file = fs::read_to_string(format!("resources/materials/{}.toml", &name));
        match file {
            Ok(file_string) => toml::from_str(&file_string).unwrap_or_default(),
            Err(_) => Material {name: name.to_string(), textures: Vec::new(), shader: String::new()},
        }
    }
    pub fn save(&self) {
        match fs::write(format!("resources/materials/{}.toml", self.name), toml::to_string(&self).expect("Failed to serialize settings!")) {
            Ok(_) => Ok(self),
            Err(_) => Err("Failed to save settings!"),
        };
    }
}
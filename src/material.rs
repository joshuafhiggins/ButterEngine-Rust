use std::{fs, io::Error};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Material {
    pub name: String,
    pub textures: Vec<(String, MagnificationFilter)>,
    pub shader: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub enum MagnificationFilter {
    #[default]
    Linear,
    Nearest,
}

pub fn to_gl_filter(filter: &MagnificationFilter) -> u32{
    match filter {
        MagnificationFilter::Linear => gl::LINEAR,
        MagnificationFilter::Nearest => gl::NEAREST,
    }
}

impl Material {
    pub fn new(name: &str) -> Result<Material, Error> {
        let file = fs::read_to_string(format!("resources/materials/{}.toml", &name));
        match file {
            Ok(file_string) => Ok(toml::from_str(&file_string).unwrap_or_default()),
            Err(error) => Err(error),
        }
    }
    // pub fn save(&self) {
    //     match fs::write(format!("resources/materials/{}.toml", self.name), toml::to_string(&self).expect("Failed to serialize settings!")) {
    //         Ok(_) => Ok(self),
    //         Err(_) => Err("Failed to save settings!"),
    //     };
    // } //TODO: Make a UI material editor
}
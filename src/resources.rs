use simple_error::*;

use std::{collections::HashMap, error::Error, sync::Arc};

use bevy_ecs::system::Resource;

use crate::{texture::{Texture, self}, shader::{Shader, self}, material::{Material, to_gl_filter, MagnificationFilter, self}, settings::Settings};

//TODO: Fix accesses
#[derive(Resource)]
pub struct Input<T> {
    keys: HashMap<T, KeyState>,
    pub xpos: f64,
    pub ypos: f64,
    pub last_xpos: f64,
    pub last_ypos: f64,
    pub cursor_mode: glfw::CursorMode,
}

#[derive(PartialEq)]
enum KeyState {
    JustPressed,
    Pressed,
    Released,
}

impl Input<glfw::Key> {
    pub fn new() -> Input<glfw::Key> {
        Input { keys: HashMap::new(), xpos: 0.0, ypos: 0.0, last_xpos: 0.0, last_ypos: 0.0, cursor_mode: glfw::CursorMode::Normal }
    }

    pub fn dispatch_keyboard(&mut self, key: glfw::Key, action: glfw::Action) {
        match action {
            glfw::Action::Release => {
                if self.keys.contains_key(&key) {
                    let value = self.keys.get_mut(&key).unwrap();
                    *value = KeyState::Released
                } else {
                    self.keys.insert(key, KeyState::Released);
                }
            },
            glfw::Action::Press => {
                if self.keys.contains_key(&key) {
                    let value = self.keys.get_mut(&key).unwrap();
                    match value {
                        KeyState::JustPressed => *value = KeyState::Pressed, //THIS CASE NEVER HAPPENS, glfw only sends one pressed event
                        KeyState::Pressed => {},
                        KeyState::Released => *value = KeyState::JustPressed,
                    }
                } else {
                    self.keys.insert(key, KeyState::JustPressed);
                }
            },
            _ => {},
        }
    }

    pub fn dispatch_mouse(&mut self, xpos: f64, ypos: f64) {
        self.xpos = xpos;
        self.ypos = ypos;
    }

    //Call this before checking for new input events
    pub fn update(&mut self) {
        for (_, v) in &mut self.keys {
            if *v == KeyState::JustPressed {
                *v = KeyState::Pressed;
            }
        }

        self.last_xpos = self.xpos;
        self.last_ypos = self.ypos;
    }

    pub fn just_pressed(&self, key: glfw::Key) -> bool {
        match self.keys.get(&key).unwrap_or(&KeyState::Released) {
            KeyState::Pressed => false,
            KeyState::JustPressed => true,
            KeyState::Released => false,
        }
    }
    pub fn pressed(&self, key: glfw::Key) -> bool {
        match self.keys.get(&key).unwrap_or(&KeyState::Released) {
            KeyState::Pressed => true,
            KeyState::JustPressed => false,
            KeyState::Released => false,
        }
    }
    pub fn released(&self, key: glfw::Key) -> bool {
        match self.keys.get(&key).unwrap_or(&KeyState::Released) {
            KeyState::Released => true,
            KeyState::JustPressed => false,
            KeyState::Pressed => false,
        }
    }
}

#[derive(Resource, Default)]
pub struct Time {
    pub delta_time: f32,
    pub last_frame: f32,
}

impl Time {
    pub fn update(&mut self, current_time: f32) {
        self.delta_time = current_time - self.last_frame;
        self.last_frame = current_time;  
    }
}

#[derive(Resource)]
pub struct WindowResource {
    pub width: i32,
    pub height: i32,
}

impl WindowResource {
    pub fn new(width: i32, height: i32) -> Self {
        WindowResource { width: width, height: height }
    }
}

//TODO: Models
#[derive(Resource, Default)]
pub struct AssetPool {
    materials: HashMap<String, Arc<Material>>,
    textures: HashMap<String, Arc<Texture>>,
    shaders: HashMap<String, Arc<Shader>>,
}

impl AssetPool {
    pub fn load_material(&mut self, name: &str, settings: &Settings) -> Result<Arc<Material>, Box<dyn Error>> {
        if self.get_material(name).is_some() {
            return Ok(self.get_material(name).unwrap().clone());
        }
        let material = Material::new(name)?;
        
        for texture in &material.textures {
            self.load_texture(&texture.0, &texture.1, settings.aniso_level)?;
        }
        self.load_shader(&material.shader)?;

        self.materials.insert(name.to_string(), Arc::new(material));
        Ok(self.get_material(name).unwrap().clone())
    }
    pub fn unload_material(&mut self, name: &str)-> Option<Box<dyn Error>> {
        let material = self.get_material(name)?;
        if Arc::strong_count(material) > 1 {
            return Some(SimpleError::new(format!("Material, {}, is still in use!", name)).into());
        }
        self.materials.remove(name);
        None
    }
    pub fn get_material(&self, name: &str) -> Option<&Arc<Material>> {
        self.materials.get(name)
    }

    pub fn load_texture(&mut self, name: &str, filter: &MagnificationFilter, aniso_level: f32) -> Result<Arc<Texture>, Box<dyn Error>> {
        if self.get_texture(name).is_some() {
            return Ok(self.get_texture(name).unwrap().clone());
        }

        let texture = Texture::new(name, material::to_gl_filter(filter), aniso_level)?;
        self.textures.insert(name.to_string(), Arc::new(texture));
        Ok(self.get_texture(name).unwrap().clone())
    }
    pub fn unload_texture(&mut self, name: &str) -> Option<Box<dyn Error>> {
        let texture = self.get_texture(name)?;
        if Arc::strong_count(texture) > 1 {
            return Some(SimpleError::new(format!("Texture, {}, is still in use!", name)).into());
        }
        self.textures.remove(name);
        None
    }
    pub fn get_texture(&self, name: &str) -> Option<&Arc<Texture>> {
        self.textures.get(name)
    }

    pub fn load_shader(&mut self, name: &str) -> Result<Arc<Shader>, Box<dyn Error>> {
        if self.get_shader(name).is_some() {
            return Ok(self.get_shader(name).unwrap().clone());
        }

        let shader = Shader::new(name)?;
        self.shaders.insert(name.to_string(), Arc::new(shader));
        Ok(self.get_shader(name).unwrap().clone())
    }
    pub fn unload_shader(&mut self, name: &str) -> Option<Box<dyn Error>> {
        let shader = self.get_shader(name)?;
        if Arc::strong_count(shader) > 1 {
            return Some(SimpleError::new(format!("Shader, {}, is still in use!", name)).into());
        }
        self.shaders.remove(name);
        None
    }
    pub fn get_shader(&self, name: &str) -> Option<&Arc<Shader>> {
        self.shaders.get(name)
    }

    pub fn unload_all(&mut self) {
        self.materials.clear();
        self.textures.clear();
        self.shaders.clear();
    }
}
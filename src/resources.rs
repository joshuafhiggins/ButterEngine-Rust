use simple_error::*;
use winit::{keyboard::KeyCode, event::ElementState};

use std::{collections::HashMap, error::Error, sync::Arc, time::*};

use bevy_ecs::system::Resource;
use winit::event::MouseButton;

use crate::{texture::{Texture}, shader::{Shader}, material::{Material, MagnificationFilter, self}, settings::Settings};

//TODO: Fix accesses
#[derive(Resource)]
pub struct Input {
    keyboard_keys: HashMap<KeyCode, KeyState>,
    mouse_buttons: HashMap<MouseButton, KeyState>,
    delta_xpos: f64,
    delta_ypos: f64,
}

#[derive(PartialEq)]
enum KeyState {
    JustPressed,
    Pressed,
    Released,
}

impl Input {
    pub fn new() -> Input {
        Input { keyboard_keys: HashMap::new(), mouse_buttons: HashMap::new(), delta_xpos: 0.0, delta_ypos: 0.0 }
    }

    pub fn dispatch_keyboard(&mut self, key: KeyCode, action: ElementState) {
        match action {
            ElementState::Released => {
                if self.keyboard_keys.contains_key(&key) {
                    let value = self.keyboard_keys.get_mut(&key).unwrap();
                    *value = KeyState::Released
                } else {
                    self.keyboard_keys.insert(key, KeyState::Released);
                }
            },
            ElementState::Pressed => {
                if self.keyboard_keys.contains_key(&key) {
                    let value = self.keyboard_keys.get_mut(&key).unwrap();
                    match value {
                        KeyState::JustPressed => *value = KeyState::Pressed, //THIS CASE NEVER HAPPENS, glfw only sends one pressed event
                        KeyState::Pressed => {},
                        KeyState::Released => *value = KeyState::JustPressed,
                    }
                } else {
                    self.keyboard_keys.insert(key, KeyState::JustPressed);
                }
            },
        }
    }

    pub fn dispatch_mouse_buttons(&mut self, button: MouseButton, action: ElementState) {
        match action {
            ElementState::Released => {
                if self.mouse_buttons.contains_key(&button) {
                    let value = self.mouse_buttons.get_mut(&button).unwrap();
                    *value = KeyState::Released
                } else {
                    self.mouse_buttons.insert(button, KeyState::Released);
                }
            },
            ElementState::Pressed => {
                if self.mouse_buttons.contains_key(&button) {
                    let value = self.mouse_buttons.get_mut(&button).unwrap();
                    match value {
                        KeyState::JustPressed => *value = KeyState::Pressed, //THIS CASE NEVER HAPPENS, glfw only sends one pressed event
                        KeyState::Pressed => {},
                        KeyState::Released => *value = KeyState::JustPressed,
                    }
                } else {
                    self.mouse_buttons.insert(button, KeyState::JustPressed);
                }
            },
        }
    }

    pub fn dispatch_mouse_motion(&mut self, delta_xpos: f64, delta_ypos: f64) {
        self.delta_xpos = delta_xpos;
        self.delta_ypos = delta_ypos;
    }

    //Call this before checking for new input events
    pub fn update(&mut self) {
        for (_, v) in &mut self.keyboard_keys {
            if *v == KeyState::JustPressed {
                *v = KeyState::Pressed;
            }
        }
    }

    pub fn keyboard_just_pressed(&self, key: KeyCode) -> bool {
        match self.keyboard_keys.get(&key).unwrap_or(&KeyState::Released) {
            KeyState::Pressed => false,
            KeyState::JustPressed => true,
            KeyState::Released => false,
        }
    }
    pub fn keyboard_pressed(&self, key: KeyCode) -> bool {
        match self.keyboard_keys.get(&key).unwrap_or(&KeyState::Released) {
            KeyState::Pressed => true,
            KeyState::JustPressed => false,
            KeyState::Released => false,
        }
    }
    pub fn keyboard_released(&self, key: KeyCode) -> bool {
        match self.keyboard_keys.get(&key).unwrap_or(&KeyState::Released) {
            KeyState::Released => true,
            KeyState::JustPressed => false,
            KeyState::Pressed => false,
        }
    }

    pub fn mouse_just_pressed(&self, button: MouseButton) -> bool {
        match self.mouse_buttons.get(&button).unwrap_or(&KeyState::Released) {
            KeyState::Pressed => false,
            KeyState::JustPressed => true,
            KeyState::Released => false,
        }
    }
    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        match self.mouse_buttons.get(&button).unwrap_or(&KeyState::Released) {
            KeyState::Pressed => true,
            KeyState::JustPressed => false,
            KeyState::Released => false,
        }
    }
    pub fn mouse_released(&self, button: MouseButton) -> bool {
        match self.mouse_buttons.get(&button).unwrap_or(&KeyState::Released) {
            KeyState::Released => true,
            KeyState::JustPressed => false,
            KeyState::Pressed => false,
        }
    }

    pub fn delta_xpos(&self) -> f64 {
        self.delta_xpos
    }
    pub fn delta_ypos(&self) -> f64 {
        self.delta_ypos
    }
}

#[derive(Resource)]
pub struct Time {
    startup: Instant,
    first_update: Option<Instant>,
    last_update: Option<Instant>,
    // pausing
    paused: bool,
    // scaling
    relative_speed: f64, // using `f64` instead of `f32` to minimize drift from rounding errors
    delta: Duration,
    delta_seconds: f32,
    delta_seconds_f64: f64,
    raw_delta: Duration,
    raw_delta_seconds: f32,
    raw_delta_seconds_f64: f64,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            startup: Instant::now(),
            first_update: None,
            last_update: None,
            paused: false,
            relative_speed: 1.0,
            delta: Duration::ZERO,
            delta_seconds: 0.0,
            delta_seconds_f64: 0.0,
            raw_delta: Duration::ZERO,
            raw_delta_seconds: 0.0,
            raw_delta_seconds_f64: 0.0,
        }
    }
}

impl Time {
    pub fn update(&mut self) {
        let now = Instant::now();

        let raw_delta = now - self.last_update.unwrap_or(self.startup);
        let delta = if self.paused {
            Duration::ZERO
        } else if self.relative_speed != 1.0 {
            raw_delta.mul_f64(self.relative_speed)
        } else {
            // avoid rounding when at normal speed
            raw_delta
        };

        if self.last_update.is_some() {
            self.delta = delta;
            self.delta_seconds = self.delta.as_secs_f32();
            self.delta_seconds_f64 = self.delta.as_secs_f64();
            self.raw_delta = raw_delta;
            self.raw_delta_seconds = self.raw_delta.as_secs_f32();
            self.raw_delta_seconds_f64 = self.raw_delta.as_secs_f64();
        } else {
            self.first_update = Some(now);
        }

        self.last_update = Some(now);
    }

        pub fn delta(&self) -> Duration {
            self.delta
        }
    
        pub fn delta_seconds(&self) -> f32 {
            self.delta_seconds
        }
    
        pub fn delta_seconds_f64(&self) -> f64 {
            self.delta_seconds_f64
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
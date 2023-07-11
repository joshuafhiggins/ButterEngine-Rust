use simple_error::*;

use std::{collections::HashMap, error::Error, sync::Arc, time::*};

use bevy_ecs::system::Resource;

use crate::{texture::{Texture}, shader::{Shader}, material::{Material, MagnificationFilter, self}, settings::Settings, window::Window};

//TODO: Fix accesses
#[derive(Resource)]
pub struct Input<T> {
    keys: HashMap<T, KeyState>,
    xpos: f64,
    ypos: f64,
    last_xpos: f64,
    last_ypos: f64,
    cursor_mode: glfw::CursorMode,
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

    pub fn xpos(&self) -> f64 {
        self.xpos
    }
    pub fn ypos(&self) -> f64 {
        self.ypos
    }
    pub fn last_xpos(&self) -> f64 {
        self.last_xpos
    }
    pub fn last_ypos(&self) -> f64 {
        self.last_ypos
    }
    pub fn cursor_mode(&self) -> glfw::CursorMode {
        self.cursor_mode
    }
    pub fn set_cursor_mode(&mut self, mode: glfw::CursorMode) {
        self.cursor_mode = mode;
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

#[derive(Resource)]
pub struct WindowResource {
    ratio: f32
}

impl WindowResource {
    pub fn new(window: &Window) -> Self {
        WindowResource { ratio: window.aspect_ratio() }
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    pub fn set_ratio(&mut self, window: &Window) {
        self.ratio = window.aspect_ratio();
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
use std::collections::HashMap;

use bevy_ecs::system::Resource;

use crate::{texture::Texture, shader::Shader, material::Material, settings::Settings};

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
    materials: HashMap<String, Material>,
    textures: HashMap<String, (Texture, u32)>,
    shaders: HashMap<String, (Shader, u32)>,
}

impl AssetPool {
    pub fn load_material(&mut self, name: &str, settings: &Settings) {
        if self.get_material(&name).is_none() {
            let material = Material::new(name);
            for texture in &material.textures {
                self.load_texture(&texture, settings.aniso_level);
            }
            self.load_shader(&material.shader);
            self.materials.insert(name.to_string(), material);
        }
    }
    pub fn unload_material(&mut self, name: &str) {
        let material = self.materials.get(name)
        .expect(format!("Material, {}, was never loaded", name).as_str());

        let mut shader = self.shaders.get_mut(&material.shader)
        .expect(format!("Shader, {}, was never loaded", material.shader).as_str());

        shader.1 = shader.1 - 1;
        if shader.1 == 0 {
            self.shaders.remove(name);
        }

        for texture_name in &material.textures {
            let mut texture = self.textures.get_mut(texture_name)
            .expect(format!("Texture, {}, was never loaded", texture_name).as_str());

            texture.1 = texture.1 - 1;
            if texture.1 == 0 {
                self.textures.remove(name);
            }
        }
    }
    pub fn get_material(&self, name: &str) -> Option<&Material> {
        self.materials.get(name)
    }

    pub fn load_texture(&mut self, name: &str, aniso_level: f32) {
        if self.get_texture(&name).is_none() {
            self.textures.insert(name.to_string(), (Texture::new(name, gl::NEAREST, aniso_level), 1)); //TODO: materials should declare texture filtering
        } else {
            let mut tup = self.textures.get_mut(name).unwrap();
            tup.1 = tup.1 + 1;
        }
    }
    pub fn unload_texture(&mut self, name: &str) {
        self.textures.remove(name);
    }
    pub fn get_texture(&self, name: &str) -> Option<&(Texture, u32)> {
        self.textures.get(name)
    }

    pub fn load_shader(&mut self, name: &str) {
        if self.get_material(&name).is_none() {
            self.shaders.insert(name.to_string(), (Shader::new(name), 1));
        } else {
            let mut tup = self.shaders.get_mut(name).unwrap();
            tup.1 = tup.1 + 1;
        }
    }
    pub fn unload_shader(&mut self, name: &str) {
        self.shaders.remove(name);
    }
    pub fn get_shader(&self, name: &str) -> Option<&(Shader, u32)> {
        self.shaders.get(name)
    }

    pub fn unload_all(&mut self) {
        self.materials.clear();

        for texture in &self.textures {
            drop(texture.1);
        }
        self.textures.clear();
        for shader in &self.shaders {
            drop(shader.1);
        }
        self.shaders.clear();
    }
}
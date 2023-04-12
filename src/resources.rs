use std::collections::HashMap;

use bevy_ecs::system::Resource;

#[derive(Resource, Default)]
pub struct Input<T> {
    keys: HashMap<T, KeyState>
}

#[derive(PartialEq)]
enum KeyState {
    JustPressed,
    Pressed,
    Released,
}

impl Input<glfw::Key> {
    pub fn new() -> Input<glfw::Key> {
        Input { keys: HashMap::new() }
    }

    pub fn dispatch(&mut self, key: glfw::Key, action: glfw::Action) {
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

    //Call this before checking for new input events
    pub fn update(&mut self) {
        for (_, v) in &mut self.keys {
            if *v == KeyState::JustPressed {
                *v = KeyState::Pressed;
            }
        }
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
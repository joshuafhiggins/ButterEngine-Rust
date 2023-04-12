use std::collections::HashMap;

use bevy_ecs::system::Resource;

#[derive(Resource, Default)]
pub struct Input<T> {
    keys: HashMap<T, KeyState>
}

#[derive(PartialEq)]
enum KeyState {
    PRESSED,
    RELEASED,
}

impl Input<glfw::Key> {
    pub fn new() -> Input<glfw::Key> {
        Input { keys: HashMap::new() }
    }

    pub fn dispatch(&mut self, key: glfw::Key, action: glfw::Action) {
        match action {
            glfw::Action::Release => {
                if self.keys.contains_key(&key) {
                    *self.keys.get_mut(&key).unwrap() = KeyState::RELEASED;
                } else {
                    self.keys.insert(key, KeyState::RELEASED);
                }
            },
            glfw::Action::Press => {
                if self.keys.contains_key(&key) {
                    *self.keys.get_mut(&key).unwrap() = KeyState::PRESSED;
                } else {
                    self.keys.insert(key, KeyState::PRESSED);
                }
            },
            _ => {},
        }
    }

    pub fn pressed(&self, key: glfw::Key) -> bool {
        match self.keys.get(&key).unwrap_or(&KeyState::RELEASED) {
            KeyState::PRESSED => true,
            KeyState::RELEASED => false,
        }
    }
    pub fn released(&self, key: glfw::Key) -> bool {
        match self.keys.get(&key).unwrap_or(&KeyState::PRESSED) {
            KeyState::RELEASED => true,
            KeyState::PRESSED => false,
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
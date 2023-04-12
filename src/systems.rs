use bevy_ecs::prelude::*;
use crate::{components::*, resources::*, settings::Settings};
use glfw::Key;

const CAMERA_SPEED: f32 = 0.05; // adjust accordingly

pub fn move_camera(mut query: Query<(&mut Position, &Rotation, &Camera)>, input: Res<Input<glfw::Key>>, time: Res<Time>) {
    for (mut position, _, camera) in &mut query {
        let move_factor = CAMERA_SPEED * time.delta_time;
        if input.pressed(Key::W) {
            position.d += move_factor * camera.front;
        }
        if input.pressed(Key::S) {
            position.d -= move_factor * camera.front;
        }
        if input.pressed(Key::A) {
            position.d -= camera.front.cross(camera.up).normalize() * move_factor;
        }
        if input.pressed(Key::D) {
            position.d += camera.front.cross(camera.up).normalize() * move_factor;
        }
    }
}

pub fn update_wireframe(input: Res<Input<glfw::Key>>, mut settings: ResMut<Settings>) {
    if input.just_pressed(glfw::Key::F5) {
        crate::renderer::toggle_wireframe(&mut settings.is_wireframe);
    }
}
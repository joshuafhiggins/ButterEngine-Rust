use bevy_ecs::prelude::*;
use crate::{components::*, resources::*};
use glfw::Key;

const cameraSpeed: f32 = 0.05; // adjust accordingly

pub fn move_camera(mut query: Query<(&mut Position, &Rotation, &Camera)>, input: Res<Input<glfw::Key>>, time: Res<Time>) {
    for (mut position, rotation, camera) in &mut query {
        let move_factor = cameraSpeed * time.delta_time;
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
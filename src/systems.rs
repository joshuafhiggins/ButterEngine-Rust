use bevy_ecs::prelude::*;
use crate::{components::*, resources::*};
use glfw::Key;

const cameraSpeed: f32 = 0.05; // adjust accordingly

pub fn move_camera(query: Query<(Entity, &Position, &Rotation, &Camera)>, input: Res<Input<glfw::Key>>) {
    for (entity, position, rotation, camera) in &query {
        if input.pressed(Key::W) {
            position.d += cameraSpeed * camera.front;
        }
        if input.pressed(Key::S) {
            position.d -= cameraSpeed * camera.front;
        }
        if input.pressed(Key::A) {
            position.d -= camera.front.cross(camera.up).normalize() * cameraSpeed;
        }
        if input.pressed(Key::D) {
            position.d += camera.front.cross(camera.up).normalize() * cameraSpeed;
        }
    }
}
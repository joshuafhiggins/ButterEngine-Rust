use bevy_ecs::{prelude::Bundle};
use crate::components::*;

#[derive(Bundle, Default)]
struct CameraBundle {
    postion: Position,
    direction: Rotation,
    camera: Camera,
}
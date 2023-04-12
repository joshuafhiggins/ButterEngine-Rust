use bevy_ecs::{prelude::Bundle};
use crate::components::*;

#[derive(Bundle, Default)]
pub struct CameraBundle {
    pub position: Position,
    pub direction: Rotation,
    pub camera: Camera,
}
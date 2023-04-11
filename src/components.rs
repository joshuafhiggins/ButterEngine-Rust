use bevy_ecs::prelude::*;
use glam::Vec3;

#[derive(Default, Component)]
pub struct Position { pub d: Vec3 }

#[derive(Default, Component)]
pub struct Rotation { d: Vec3 }

#[derive(Default, Component)]
pub struct Scale { d: Vec3 }

#[derive(Default, Component)]
pub struct Camera { 
    front: Vec3,
    up: Vec3,
}
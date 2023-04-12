use bevy_ecs::prelude::*;
use glam::Vec3;

#[derive(Default, Component)]
pub struct Position { pub d: Vec3 }

#[derive(Default, Component)]
pub struct Rotation { pub d: Vec3 }

#[derive(Default, Component)]
pub struct Scale { pub d: Vec3 }

#[derive(Default, Component)]
pub struct Camera { 
    pub front: Vec3,
    pub up: Vec3,
}
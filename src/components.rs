use bevy_ecs::prelude::*;
use glam::{Vec3, Mat4};

#[derive(Default, Component)]
pub struct Position { pub d: Vec3 }

#[derive(Default, Component)]
pub struct Rotation { pub d: Vec3 }

#[derive(Default, Component)]
pub struct Scale { pub d: Vec3 }

//TODO: Redo accesses
#[derive(Default, Component)]
pub struct Camera { 
    pub front: Vec3,
    pub up: Vec3,
    pub yaw: f64,
    pub pitch: f64,
    pub first_mouse: bool,
    pub view: glam::Mat4,
    pub projection: glam::Mat4,
}

impl Camera {
    pub fn get_calculation(&self) -> Mat4 {
        return self.projection * self.view;
    }

    pub fn set_projection(&mut self, fov: f32, ratio: f32, near_plane: f32, far_plane: f32) {
        self.projection = Mat4::perspective_rh_gl(fov, ratio, near_plane, far_plane);
    }
}
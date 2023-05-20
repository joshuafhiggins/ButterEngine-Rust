use std::ptr;

use bevy_ecs::prelude::Component;

use crate::{renderer::{VAO, VBO, IBO, GPUObject}};

#[derive(Component)]
pub struct Mesh {
    vao: VAO,
    ibo: IBO,
    buffers: Vec<VBO>,
    pub texture: String, //TODO: Change to material
    pub shader: String,
}

impl Mesh {
    pub fn new(indices: Vec<i32>, texture: &str, shader: &str) -> Mesh {
        let vao: VAO = VAO::new();
        let ibo: IBO = IBO::new(indices, &vao);
        let buffers: Vec<VBO> = Vec::new();

        return Mesh { vao, ibo, buffers, texture: texture.to_string(), shader: shader.to_string()};
    }
    pub fn add_buffer(&mut self, data: Vec<f32>, index: u32, size: i32) {
        self.buffers.push(VBO::new(data, index, size, &self.vao));
    }
    pub fn render(&self) {
        self.vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.ibo.get_indices().len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
        self.vao.unbind();
    }
}
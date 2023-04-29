use std::ptr;

use crate::{renderer::{VAO, VBO, IBO, GPUObject}, shader::Shader};

pub struct Mesh<'a> {
    vao: VAO,
    ibo: IBO,
    buffers: Vec<VBO>,
    shader: &'a Shader,
    //TODO: Make material struct that will have shader
} //TODO: Make into component for ECS and render in a system

impl Mesh<'_> {
    pub fn new(indices: Vec<i32>, shader: &Shader) -> Mesh<'_> {
        let vao: VAO = VAO::new();
        let ibo: IBO = IBO::new(indices, &vao);
        let buffers: Vec<VBO> = Vec::new();

        return Mesh { vao, ibo, buffers, shader };
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
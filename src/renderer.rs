use gl::types::*;
use std::ptr;
use std::mem;
use std::os::raw::c_void;

pub struct VAO {
    id: u32,
}

impl VAO {
    pub fn new() -> VAO{
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        return VAO {id: vao};
    }
}

impl BufferObject for VAO {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn cleanup(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

pub struct VBO {
    pub id: u32,
    pub data: Vec<f32>
}

impl VBO {
    pub fn new(data: Vec<f32>, index: u32, size: i32, vao: &VAO) -> VBO {
        let mut vbo: VBO = VBO {id: 0, data: data};
        unsafe {
            gl::GenBuffers(1, &mut vbo.id);
        }
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        vao.bind();
        vbo.bind();
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER,
                        (vbo.data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        vbo.data.get(0).unwrap() as *const f32 as *const c_void,
                        gl::STATIC_DRAW);

            gl::VertexAttribPointer(index, size, gl::FLOAT, gl::FALSE, size * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(index);
        }
        vbo.unbind();
        vao.unbind();

        return vbo;
    }
    pub fn get_data(&self) -> &Vec<f32> {
        &self.data
    }
}

impl BufferObject for VBO {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn cleanup(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct IBO {
    id: u32,
    indices: Vec<i32>
}

impl IBO {
    pub fn new(indices: Vec<i32>, vao: &VAO) -> IBO {
        let mut ibo: IBO = IBO {id: 0, indices};
        unsafe {
            gl::GenBuffers(1, &mut ibo.id);
        }
        vao.bind();
        ibo.bind();
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                (ibo.indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                ibo.indices.get(0).unwrap() as *const i32 as *const c_void,
                gl::STATIC_DRAW);
        }
        //never unbind
        //ibo.unbind();
        vao.unbind();

        return ibo;
    }
    pub fn get_indices(&self) -> &Vec<i32> {
        &self.indices
    }
}

impl BufferObject for IBO {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    fn cleanup(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub trait BufferObject {
    fn bind(&self);
    fn unbind(&self);
    fn cleanup(&self);
}

pub fn toggle_wireframe(is_wireframe: &mut bool) {
    *is_wireframe = !*is_wireframe;
    update_wireframe(&is_wireframe);
}

pub fn update_wireframe(is_wireframe: &bool) {
    match is_wireframe {
        true => unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        },
        false => unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        },
    }
}
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use cgmath::*;
use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

pub struct Shader {
    name: String,
    program: u32
}

impl Shader {
    pub fn new(name: String) -> Shader {
        let vertex_src: String = fs::read_to_string(format!("resources/shaders/{}.vs", name)).expect(format!("Failed to load shader file at resources/shaders/{}.vs!", name).as_str());
        let fragment_src: String = fs::read_to_string(format!("resources/shaders/{}.fs", name)).expect(format!("Failed to load shader file at resources/shaders/{}.fs!", name).as_str());
        
        let program = unsafe {
            // build and compile our shader program
            // ------------------------------------
            // vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_src.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
    
            // check for shader compile errors
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }
    
            // fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_src.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            // check for shader compile errors
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }
    
            // link shaders
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            // check for linking errors
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader_program
        };
        
        return Shader {name: name, program: program};
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }

    //Bools
    pub fn set_uniform_bool(&self, label: String, value: &bool) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform1i(location, *value as i32);
        }
    }

    //Floats
    pub fn set_uniform_4f(&self, label: String, value: &Vector4<f32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform4f(location, value.x, value.y, value.z, value.w);
        }
    }
    pub fn set_uniform_3f(&self, label: String, value: &Vector3<f32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }
    pub fn set_uniform_2f(&self, label: String, value: &Vector2<f32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform2f(location, value.x, value.y);
        }
    }
    pub fn set_uniform_1f(&self, label: String, value: &f32) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform1f(location, *value);
        }
    }

    //Signed Integers
    pub fn set_uniform_4i(&self, label: String, value: &Vector4<i32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform4i(location, value.x, value.y, value.z, value.w);
        }
    }
    pub fn set_uniform_3i(&self, label: String, value: &Vector3<i32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform3i(location, value.x, value.y, value.z);
        }
    }
    pub fn set_uniform_2i(&self, label: String, value: &Vector2<i32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform2i(location, value.x, value.y);
        }
    }
    pub fn set_uniform_i32(&self, label: String, value: &i32) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform1i(location, *value);
        }
    }

    //Unsigned Integers
    pub fn set_uniform_4ui(&self, label: String, value: &Vector4<u32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform4ui(location, value.x, value.y, value.z, value.w);
        }
    }
    pub fn set_uniform_3ui(&self, label: String, value: &Vector3<u32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform3ui(location, value.x, value.y, value.z);
        }
    }
    pub fn set_uniform_2ui(&self, label: String, value: &Vector2<u32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform2ui(location, value.x, value.y);
        }
    }
    pub fn set_uniform_u32(&self, label: String, value: &u32) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform1ui(location, *value);
        }
    }

    //Doubles
    pub fn set_uniform_4d(&self, label: String, value: &Vector4<f64>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform4d(location, value.x, value.y, value.z, value.w);
        }
    }
    pub fn set_uniform_3d(&self, label: String, value: &Vector3<f64>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform3d(location, value.x, value.y, value.z);
        }
    }
    pub fn set_uniform_2d(&self, label: String, value: &Vector2<f64>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform2d(location, value.x, value.y);
        }
    }
    pub fn set_uniform_d64(&self, label: String, value: &f64) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform1d(location, *value);
        }
    }

    //Matrices Floats
    pub fn set_uniform_2x2f(&self, label: String, count: Option<i32>, value: &Matrix2<f32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::UniformMatrix2fv(location, count.unwrap_or(1), gl::FALSE, value.as_ptr());
        }
    }
    pub fn set_uniform_3x3f(&self, label: String, count: Option<i32>, value: &Matrix3<f32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::UniformMatrix3fv(location, count.unwrap_or(1), gl::FALSE, value.as_ptr());
        }
    }
    pub fn set_uniform_4x4f(&self, label: String, count: Option<i32>, value: &Matrix4<f32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::UniformMatrix4fv(location, count.unwrap_or(1), gl::FALSE, value.as_ptr());
        }
    }

    //Matrices Doubles
    pub fn set_uniform_2x2d(&self, label: String, count: Option<i32>, value: &Matrix2<f64>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::UniformMatrix2dv(location, count.unwrap_or(1), gl::FALSE, value.as_ptr());
        }
    }
    pub fn set_uniform_3x3d(&self, label: String, count: Option<i32>, value: &Matrix3<f64>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::UniformMatrix3dv(location, count.unwrap_or(1), gl::FALSE, value.as_ptr());
        }
    }
    pub fn set_uniform_4x4d(&self, label: String, count: Option<i32>, value: &Matrix4<f64>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::UniformMatrix4dv(location, count.unwrap_or(1), gl::FALSE, value.as_ptr());
        }
    }

    //TODO: Direct State Access
}
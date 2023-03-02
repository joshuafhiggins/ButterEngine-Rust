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

    pub fn set_uniform_4f32(&self, label: String, vec4f: Vector4<f32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform4f(location, vec4f.x, vec4f.y, vec4f.z, vec4f.w);
        }
    }

    pub fn set_uniform_3f32(&self, label: String, vec3f: Vector3<f32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform3f(location, vec3f.x, vec3f.y, vec3f.z);
        }
    }

    pub fn set_uniform_2f32(&self, label: String, vec2f: Vector2<f32>) {
        let c_label = CString::new(label).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, c_label.as_ptr());
            gl::Uniform2f(location, vec2f.x, vec2f.y);
        }
    }
}
use std::{io::Read, os::raw::c_void, fs::File};

use crate::renderer::{self, GPUObject};

pub struct Texture {
    handle: u32,
}

impl Texture {
    pub fn new(name: &str, mag_filter: u32, aniso_level: f32) -> Texture {
        let mut texture: Texture = Texture { handle: 0 };
        let image: Image = Image::new(&name);

        unsafe {
            gl::GenTextures(1, &mut texture.handle);
        }
        texture.bind();
        unsafe {
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAX_ANISOTROPY_EXT, aniso_level);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
 
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                image.componenets,
                image.width,
                image.height,
                0,
                image.opengl_load_type,
                gl::UNSIGNED_BYTE,
                image.data as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        texture.unbind();

        return texture;
    }
}

impl renderer::GPUObject for Texture {
    fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
    }
}

struct Image {
    width: i32,
    height: i32,
    componenets: i32,
    opengl_load_type: u32,
    data: *mut u8,
}

impl Image {
    pub fn new(name: &str) -> Image {
        let mut image: Image = Image { width: 0, height: 0, componenets: 0, opengl_load_type: gl::RGB, data: 0 as *mut u8 };

        // Load file into memory
        let mut f = File::open(format!("resources/textures/{}.png", name)).expect("file not found");
        let mut contents = vec![];
        f.read_to_end(&mut contents).expect("Failed to put data into Vec<u8> in texture!");

        unsafe {
            // load image, create texture and generate mipmaps
            stb_image_rust::stbi_set_flip_vertically_on_load(true as i32);
            image.data = stb_image_rust::stbi_load_from_memory(
                contents.as_mut_ptr(),
                contents.len() as i32,
                &mut image.width,
                &mut image.height,
                &mut image.componenets,
                0,
            );
        }
        image.opengl_load_type = match image.componenets {
            1 => gl::RED,
            2 => gl::RG,
            3 => gl::RGB,
            4 => gl::RGBA,
            _ => gl::RGB
        };
        return image;
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            stb_image_rust::c_runtime::free(self.data);
        }
    }
}
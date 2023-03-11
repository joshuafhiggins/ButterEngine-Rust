use std::{io::Read, os::raw::c_void, fs::File};

use crate::renderer::{self, GPUObject};

pub struct Texture {
    handle: u32,
}

impl Texture {
    pub fn new(name: String) -> Texture {
        let mut texture: Texture = Texture { handle: 0 };

        unsafe {
            gl::GenTextures(1, &mut texture.handle);
        }
        texture.bind();
        unsafe {
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        // Load file into memory
        let mut f = File::open(format!("resources/textures/{}.png", name)).expect("file not found");
        let mut contents = vec![];
        f.read_to_end(&mut contents);

	    // Load the image
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut comp: i32 = 0;
        let img: *mut u8;

        unsafe {
            // load image, create texture and generate mipmaps
            stb_image_rust::stbi_set_flip_vertically_on_load(true as i32);
            img = stb_image_rust::stbi_load_from_memory(
                contents.as_mut_ptr(),
                contents.len() as i32,
                &mut width,
                &mut height,
                &mut comp,
                0,
            );
            
            let image_load_type = match comp {
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => gl::RGB
            };

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                comp,
                width,
                height,
                0,
                image_load_type,
                gl::UNSIGNED_BYTE,
                img as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        texture.unbind();
        unsafe {
            stb_image_rust::c_runtime::free(img);
        }
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

    fn cleanup(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
    }
}

extern crate gl_generator;

use gl_generator::{Registry, Fallbacks, GlobalGenerator, Api, Profile};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, [
        "GL_EXT_texture_filter_anisotropic", // additional extension we want to use
    ])
        .write_bindings(
            GlobalGenerator,
            &mut file_gl
        )
        .unwrap();
}
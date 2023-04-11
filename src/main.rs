mod renderer;
mod settings;
mod shader;
mod texture;
mod window;
mod components;
mod entities;

use renderer::GPUObject;
use renderer::IBO;
use renderer::VAO;
use renderer::VBO;
use settings::Settings;
use shader::Shader;
use std::ptr;
use texture::Texture;
use window::Window;
use bevy_ecs::world::World;

fn main() {
    let mut settings: Settings = settings::load();
    let mut window = Window::new(
        settings.width,
        settings.height,
        &settings.title,
        settings.swap_interval,
    );
    window.center();
    window.init_gl();
    renderer::update_wireframe(&settings.is_wireframe);

    let shader: Shader = Shader::new("triangle".to_string());

    let vertices: [f32; 12] = [
            // positions
             0.5,  0.5, 0.0, // top right
             0.5, -0.5, 0.0, // bottom right
            -0.5, -0.5, 0.0, // bottom left
            -0.5,  0.5, 0.0, // top left
    ];
    let colors: [f32; 12] = [
        // texture coords
        1.0, 0.0, 0.0, // top right
        0.0, 1.0, 0.0, // bottom right
        0.0, 0.0, 1.0, // bottom left
        1.0, 1.0, 0.0 // top left
    ];
    let texture_coords: [f32; 8] = [
        // texture coords
        1.0, 1.0, // top right
        1.0, 0.0, // bottom right
        0.0, 0.0, // bottom left
        0.0, 1.0  // top left
    ];
    let indices = [
        0, 1, 3, // first Triangle
        1, 2, 3, // second Triangle
    ];

    let vao: VAO = VAO::new();
    let vbo: VBO = VBO::new(vertices.to_vec(), 0, 3, &vao);
    let cbo: VBO = VBO::new(colors.to_vec(), 1, 3, &vao);
    let tbo: VBO = VBO::new(texture_coords.to_vec(), 2, 2, &vao);
    let ibo: IBO = IBO::new(indices.to_vec(), &vao);
    let texture: Texture = Texture::new("planks_oak".to_string(), gl::NEAREST);

    let world = World::default();

    while !window.should_close() {
        //TODO: Render
        //TODO: ecs/physics/sound/ui/engine updates

        window.process_events(&mut settings);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        shader.bind();
        texture.bind();
        vao.bind();
        unsafe {
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            //TODO: Create a Mesh struct
            gl::DrawElements(
                gl::TRIANGLES,
                ibo.get_indices().len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
        vao.unbind();
        texture.unbind();
        shader.unbind();

        window.swap_buffers();
        window.poll_events();
    }

    settings::save(settings).expect("Unable to save settings!");
}

mod window;
mod shader;
mod renderer;
mod settings;

use std::ptr;
use renderer::BufferObject;
use renderer::IBO;
use renderer::VAO;
use renderer::VBO;
use settings::Settings;
use window::Window;
use shader::Shader;

fn main() {
    let mut settings: Settings = settings::load();
    let mut window = Window::new(settings.width, settings.height, &settings.title, settings.swap_interval);
    window.center();
    window.init_gl();
    renderer::update_wireframe(&settings.is_wireframe);

    let shader: Shader = Shader::new("triangle".to_string());

    let vertices: [f32; 12] = [
             0.5,  0.5, 0.0,  // top right
             0.5, -0.5, 0.0,  // bottom right
            -0.5, -0.5, 0.0,  // bottom left
            -0.5,  0.5, 0.0   // top left
        ];
    let indices: [i32; 6] = [ // note that we start from 0!
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];

    let vao: VAO = VAO::new();
    let vbo: VBO = VBO::new(vertices.to_vec(), 0, 3, &vao);
    let ibo: IBO = IBO::new(indices.to_vec(), &vao);

    while !window.should_close() {
        //TODO: Render
        //TODO: ecs/physics/sound/ui/engine updates

        window.process_events(&mut settings);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        shader.bind();
        vao.bind();
        unsafe {
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            //TODO: Create a Mesh struct
            gl::DrawElements(gl::TRIANGLES, ibo.get_indices().len() as i32, gl::UNSIGNED_INT, ptr::null());
        }
        vao.unbind();
        shader.unbind();

        window.swap_buffers();
        window.poll_events();
    }

    vbo.cleanup();
    ibo.cleanup();
    vao.cleanup();
    shader.cleanup();

    settings::save(settings).expect("Unable to save settings!");
}



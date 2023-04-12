mod renderer;
mod settings;
mod shader;
mod texture;
mod window;
mod components;
mod entities;
mod systems;
mod resources;

use bevy_ecs::schedule::Schedule;
use components::Camera;
use components::Position;
use components::Rotation;
use glam::Vec3;
use renderer::GPUObject;
use renderer::IBO;
use renderer::VAO;
use renderer::VBO;
use resources::Input;
use settings::Settings;
use shader::Shader;
use std::ptr;
use texture::Texture;
use window::Window;
use bevy_ecs::world::World;
use entities::*;
use resources::*;

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

    let mut world = World::new();

    let mut pre_sys = Schedule::default();
    let mut sys_gl = Schedule::default();
    sys_gl.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    let mut sys = Schedule::default();
    let mut post_sys = Schedule::default();


    world.spawn(CameraBundle {
        position: Position { d: Vec3::new(0.0, 0.0, 3.0) },
        direction: Rotation::default(),
        camera: Camera {front: Vec3::new(0.0, 0.0, -1.0), up: Vec3::new(0.0, 1.0, 0.0) }
    });
    world.insert_resource(Input::new());
    world.insert_resource(Time::default());
    world.insert_resource(settings);

    sys.add_system(systems::move_camera);
    sys_gl.add_system(systems::update_wireframe);

    while !window.should_close() {
        window.update(&mut world);

        pre_sys.run(&mut world);
        sys_gl.run(&mut world);
        sys.run(&mut world);

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

        post_sys.run(&mut world);

        window.swap_buffers();
    }

    settings::save(world.get_resource::<Settings>().unwrap()).expect("Unable to save settings!");
}

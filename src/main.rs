mod components;
mod entities;
mod renderer;
mod resources;
mod settings;
mod shader;
mod systems;
mod texture;
mod window;

use bevy_ecs::schedule::Schedule;
use bevy_ecs::world::World;
use components::*;
use entities::*;
use glam::*;
use renderer::*;
use resources::*;
use settings::Settings;
use shader::Shader;
use std::ptr;
use texture::Texture;
use window::Window;

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

    let shader: Shader = Shader::new("default".to_string());

    let vertices: [f32; 15] = [
        -0.5, 0.0,  0.5,     	
        -0.5, 0.0, -0.5,     	
         0.5, 0.0, -0.5,     	
         0.5, 0.0,  0.5,     	
         0.0, 0.8,  0.0,     	
    ];
    let colors: [f32; 15] = [
        0.83, 0.70, 0.44,
        0.83, 0.70, 0.44,
        0.83, 0.70, 0.44,
        0.83, 0.70, 0.44,
        0.92, 0.86, 0.76,
    ];
    let texture_coords: [f32; 10] = [
        0.0, 0.0,
        5.0, 0.0,
        0.0, 0.0,
        5.0, 0.0,
        2.5, 5.0,
    ];
    let indices = [
        0, 1, 2,
        0, 2, 3,
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
        3, 0, 4,
    ];

    let vao: VAO = VAO::new();
    let _: VBO = VBO::new(vertices.to_vec(), 0, 3, &vao);
    let _: VBO = VBO::new(colors.to_vec(), 1, 3, &vao);
    let _: VBO = VBO::new(texture_coords.to_vec(), 2, 2, &vao);
    let ibo: IBO = IBO::new(indices.to_vec(), &vao);
    let texture: Texture = Texture::new("planks_oak".to_string(), gl::NEAREST);

    let mut world = World::new();

    let mut pre_sys = Schedule::default();
    let mut sys_gl = Schedule::default();
    sys_gl.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    let mut sys = Schedule::default();
    let mut post_sys = Schedule::default();

    let camera = world.spawn(CameraBundle {
        position: Position {
            d: Vec3::new(0.0, 0.0, 3.0),
        },
        direction: Rotation::default(),
        camera: Camera {
            front: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            first_mouse: true,
            view: Mat4::IDENTITY,
            projection: Mat4::perspective_rh_gl(
            90.0_f32.to_radians(), 
            window.handle.get_framebuffer_size().0 as f32 / window.handle.get_framebuffer_size().1 as f32, 
            0.01, 
            100.0),
        },
    }).id();

    world.insert_resource(Input::new());
    world.insert_resource(Time::default());
    world.insert_resource(settings);
    world.insert_resource(WindowResource::new(window.handle.get_framebuffer_size().0, window.handle.get_framebuffer_size().1));

    sys.add_system(systems::move_camera);
    sys.add_system(systems::update_projection);
    sys_gl.add_system(systems::update_wireframe);

    while !window.should_close() {
        pre_sys.run(&mut world);
        sys_gl.run(&mut world);
        sys.run(&mut world);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        shader.bind();

        let camera_component = world.entity(camera).get::<Camera>().unwrap();
        shader.set_uniform_4x4f("camMatrix".to_string(), None, &camera_component.get_calculation());

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

        window.update(&mut world);
        window.swap_buffers();
    }

    settings::save(world.get_resource::<Settings>().unwrap()).expect("Unable to save settings!");
}

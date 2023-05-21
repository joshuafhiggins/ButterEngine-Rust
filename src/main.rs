#![allow(dead_code)]

mod components;
mod entities;
mod renderer;
mod resources;
mod settings;
mod shader;
mod systems;
mod texture;
mod window;
mod mesh;
mod material;

use bevy_ecs::schedule::Schedule;
use bevy_ecs::world::World;
use components::*;
use entities::*;
use glam::*;
use material::{Material, MagnificationFilter};
use mesh::Mesh;
use resources::*;
use settings::Settings;
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

    let mut asset_pool = AssetPool::default();
    asset_pool.load_material("wood", &settings);

    let mut mesh: Mesh = Mesh::new(indices.to_vec(), "wood");
    
    mesh.add_buffer(vertices.to_vec(), 0, 3);
    mesh.add_buffer(colors.to_vec(), 1, 3);
    mesh.add_buffer(texture_coords.to_vec(), 2, 2);

    let mut world = World::new();

    let mut preupdate_sys = Schedule::default();
    let mut gl_sys = Schedule::default();
    gl_sys.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    let mut update_sys = Schedule::default();
    let mut render_sys = Schedule::default();
    render_sys.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    let mut postupdate_sys = Schedule::default();

    let _ = world.spawn(CameraBundle {
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

    world.spawn(mesh);

    world.insert_resource(Input::new());
    world.insert_resource(Time::default());
    world.insert_resource(settings);
    world.insert_resource(WindowResource::new(window.handle.get_framebuffer_size().0, window.handle.get_framebuffer_size().1));
    world.insert_resource(asset_pool);

    update_sys.add_system(systems::move_camera);
    update_sys.add_system(systems::update_projection);
    gl_sys.add_system(systems::update_wireframe);
    render_sys.add_system(systems::render_scene);

    while !window.should_close() {
        preupdate_sys.run(&mut world);
        gl_sys.run(&mut world);
        update_sys.run(&mut world);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render_sys.run(&mut world);

        postupdate_sys.run(&mut world);

        window.update(&mut world);
        window.swap_buffers();
    }

    settings::save(world.get_resource::<Settings>().unwrap()).expect("Unable to save settings!");
}

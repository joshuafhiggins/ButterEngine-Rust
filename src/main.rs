//#![allow(dead_code)]

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
use mesh::Mesh;
use resources::*;
use settings::Settings;
use window::Window;
use winit::event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::KeyCode;

fn main() {
    let settings: Settings = settings::load();
    let event_loop = EventLoop::new();
    let (window, gl_context) = Window::new(
        settings.width,
        settings.height,
        &settings.title,
        &event_loop,
    );
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
    let _ = asset_pool.load_material("wood", &settings);

    let mut mesh: Mesh = Mesh::new(indices.to_vec(), "wood");
    
    mesh.add_buffer(vertices.to_vec(), 0, 3);
    mesh.add_buffer(colors.to_vec(), 1, 3);
    mesh.add_buffer(texture_coords.to_vec(), 2, 2);

    let mut world = World::new();

    let mut opengl_update = Schedule::default();
    let mut update = Schedule::default();
    let mut opengl_render = Schedule::default();

    opengl_update.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    opengl_render.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);

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
            window.aspect_ratio(), 
            0.01, 
            100.0),
        },
    }).id();

    world.spawn(mesh);

    world.insert_resource(Input::new());
    world.insert_resource(Time::default());
    world.insert_resource(settings);
    world.insert_resource(window);
    world.insert_resource(asset_pool);

    update.add_system(systems::move_camera);
    update.add_system(systems::update_projection);
    opengl_update.add_system(systems::update_wireframe);
    opengl_render.add_system(systems::render_scene);

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        control_flow.set_poll();
    
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                exit(control_flow, &mut world);
            },
            Event::MainEventsCleared => {
                let before = std::time::Instant::now();

                // Application update code.
                let mut input = world.get_resource_mut::<Input>().unwrap();
                input.update(); //Must come before our input.dispatch()
                let mut window = world.get_resource_mut::<Window>().unwrap();
                window.update();
                let mut time = world.get_resource_mut::<Time>().unwrap();
                time.update();

                update.run(&mut world);
                opengl_update.run(&mut world);
        
                // Render
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
                opengl_render.run(&mut world);
                gl_context.handle.swap_buffers();

                let after = std::time::Instant::now();
        
                let settings = world.get_resource_mut::<Settings>().unwrap();
                let time_spent = after.duration_since(before);
                let budget = std::time::Duration::new(0, 1_000_000_000u32 / settings.swap_interval as u32);
                ::std::thread::sleep(budget.saturating_sub(time_spent));
            },

            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::KeyboardInput { event: KeyEvent { physical_key: KeyCode::Escape, state: ElementState::Pressed, .. }, .. } => {
                        exit(control_flow, &mut world);
                    }
                    WindowEvent::KeyboardInput { event: KeyEvent { physical_key, state, .. }, .. } => {
                        let mut input = world.get_resource_mut::<Input>().unwrap();
                        input.dispatch_keyboard(physical_key, state);
                    }
                    WindowEvent::Resized(size) => {
                        unsafe {
                            gl::Viewport(0, 0, size.width as i32, size.height as i32);
                        }
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        let mut input = world.get_resource_mut::<Input>().unwrap();
                        input.dispatch_mouse_buttons(button, state);
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent { event, .. } => {
                match event {
                    DeviceEvent::MouseMotion { delta } => {
                        let mut input = world.get_resource_mut::<Input>().unwrap();
                        input.dispatch_mouse_motion(delta.0, delta.1);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

fn exit(control_flow: &mut ControlFlow, world: &mut World) {
    settings::save(world.get_resource::<Settings>().unwrap()).expect("Unable to save settings!");
    println!("Stopping...");
    control_flow.set_exit();
}

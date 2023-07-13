use std::{sync::mpsc::Receiver, ffi::c_int};
use bevy_ecs::prelude::*;
use raw_gl_context::{GlContext, GlConfig, Profile};

use crate::resources::*;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder, dpi::{Size, PhysicalSize, PhysicalPosition},
};

#[derive(Resource)]
pub struct Window {
    handle: winit::window::Window,
    //context: GlContext,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_visible(false) //Make sure to make visible
            .with_title(title.to_string())
            .with_inner_size(PhysicalSize::new(width, height))
            .build(&event_loop).unwrap();

        // let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        // let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
        //     .expect("Failed to create GLFW window.");

        let context = unsafe { 
            GlContext::create(&window, GlConfig {
                version: (4, 6),
                #[cfg(target_os = "macos")]
                profile: Profile::Compatibility,
                profile: Profile::Core,
                red_bits: 8,
                blue_bits: 8,
                green_bits: 8,
                alpha_bits: 8,
                depth_bits: 24,
                stencil_bits: 8,
                samples: None,
                srgb: true,
                double_buffer: true,
                vsync: false,
            }).unwrap()
        };

        unsafe {
            context.make_current();
        }
    
        gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

        let mut our_window = Window { handle: window };

        our_window.center();
        our_window.init_gl();
        
        our_window
    }

    pub fn center(&mut self) {
        let win_size = self.handle.inner_size();
        let monitor_size = self.handle.primary_monitor().unwrap().size();
        self.handle.set_outer_position(PhysicalPosition::new(
            (monitor_size.width - win_size.width) / 2, 
            (monitor_size.height - win_size.height) / 2));
    }

    pub fn swap_buffers(&mut self) {
        self.handle.swap_buffers();
    }

    pub fn update(&mut self, world: &mut World) {
        let mut input = world.get_resource_mut::<Input<glfw::Key>>().unwrap();
        input.update(); //Must come before our input.dispatch()

        self.glfw.poll_events();

        self.handle_window_events(world);

        let mut time = world.get_resource_mut::<Time>().unwrap();
        time.update();
    }

    pub fn init_gl(&self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Enable(gl::DEPTH_TEST);
        }
        //TODO: More GL init please
    }

    pub fn should_close(&self) -> bool {
        self.handle.should_close()
    }

    pub fn aspect_ratio(&self) -> f32 {
        let size = self.handle.inner_size();
        size.width as f32 / size.height as f32
    }

    fn handle_window_events(&mut self, world: &mut World) {
        for (_, event) in glfw::flush_messages(&self.events) { //TODO: Handle all events
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.handle.set_should_close(true);
                }
                glfw::WindowEvent::MouseButton(glfw::MouseButton::Button1, glfw::Action::Press, _) => {
                    let mut input = world.get_resource_mut::<Input<glfw::Key>>().unwrap();
                    self.handle.set_cursor_mode(glfw::CursorMode::Disabled);
                    input.set_cursor_mode(glfw::CursorMode::Disabled);
                }
                glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) => {
                    let mut input = world.get_resource_mut::<Input<glfw::Key>>().unwrap();
                    self.handle.set_cursor_mode(glfw::CursorMode::Normal);
                    input.set_cursor_mode(glfw::CursorMode::Normal);
                }
                glfw::WindowEvent::Key(a, _, b, _) => {
                    let mut input = world.get_resource_mut::<Input<glfw::Key>>().unwrap();
                    input.dispatch_keyboard(a, b);
                }
                glfw::WindowEvent::CursorPos(a, b) => {
                    let mut input = world.get_resource_mut::<Input<glfw::Key>>().unwrap();
                    input.dispatch_mouse(a, b);
                }
                glfw::WindowEvent::Size(a, b) => {
                    let mut win_resource = world.get_resource_mut::<WindowResource>().unwrap();
                    win_resource.set_ratio(self);
                    unsafe {
                        gl::Viewport(0, 0, a, b);
                    }
                }
                _ => {}
            }
        }
    }
}


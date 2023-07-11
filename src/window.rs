use std::{sync::mpsc::Receiver, ffi::c_int};
use bevy_ecs::prelude::*;
use glfw::{Context, ffi};

use crate::resources::*;

pub struct Window {
    handle: glfw::Window,
    glfw: glfw::Glfw,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        window.glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        window.glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        window.make_current();
        window.set_all_polling(true);

        gl::load_with(|s| window.get_proc_address(s) as * const _);

        let mut our_window = Window { handle: window, glfw: glfw, events: events };

        our_window.center();
        our_window.init_gl();
        our_window.set_swap_interval(0);
        
        our_window
    }

    pub fn center(&mut self) {
        let (win_width, win_height) = self.handle.get_size();
        self.glfw.with_primary_monitor(|_, monitor| {
           let vid_mode = monitor.expect("Unable to get the primrary monitor!").get_video_mode().expect("Unable to get the VidMode of the primrary monitor!");
           self.handle.set_pos((vid_mode.width as i32 - win_width) / 2, (vid_mode.height as i32 - win_height) / 2);
        });
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

    fn set_swap_interval(&self, interval: i32) {
        unsafe {
            ffi::glfwSwapInterval(interval as c_int);
        }
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
        self.handle.get_framebuffer_size().0 as f32 / self.handle.get_framebuffer_size().1 as f32
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


use std::{sync::mpsc::Receiver, ffi::c_int};
use bevy_ecs::prelude::*;
use glfw::{Context, ffi};

use crate::resources::*;

pub struct Window {
    pub handle: glfw::Window,
    pub glfw: glfw::Glfw,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str, framerate: i32) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        let mut our_window = Window { handle: window, glfw: glfw, events: events};

        our_window.glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        our_window.glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        our_window.glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        our_window.handle.make_current();
        our_window.handle.set_all_polling(true);
        our_window.set_swap_interval(framerate);

        return our_window;
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
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            handle_window_event(&mut self.handle, event, world);
        }

        let mut time = world.get_resource_mut::<Time>().unwrap();
        time.update(self.handle.glfw.get_time() as f32);
    }

    pub fn set_swap_interval(&mut self, interval: i32) {
        unsafe {
            ffi::glfwSwapInterval(interval as c_int);
        }
    }

    pub fn init_gl(&mut self) {
        gl::load_with(|s| self.handle.get_proc_address(s) as * const _);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }
        //TODO: More GL init please
    }

    pub fn should_close(&self) -> bool {
        self.handle.should_close()
    }
 
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, world: &mut World) {
    //TODO: Handle all events
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(a, _, b, _) => {
            let mut input = world.get_resource_mut::<Input<glfw::Key>>().unwrap();
            input.dispatch(a, b);
        }
        // glfw::WindowEvent::Key(glfw::Key::F5, _, glfw::Action::Press, _) => {
        //     crate::renderer::toggle_wireframe(&mut settings.is_wireframe);
        // }
        _ => {}
    }
}
use std::sync::mpsc::Receiver;

use glfw::Context;

pub struct Window {
    pub handle: glfw::Window,
    pub glfw: glfw::Glfw,
    pub events: Receiver<(f64, glfw::WindowEvent)>
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str, mode: glfw::WindowMode, framerate: glfw::SwapInterval) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw.create_window(width, height, title, mode)
            .expect("Failed to create GLFW window.");
    
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        window.make_current();
        window.set_all_polling(true);
        window.glfw.set_swap_interval(framerate);

        return Window { handle: window, glfw: glfw, events: events };
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

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    pub fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            handle_window_event(&mut self.handle, event);
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

fn handle_window_event(handle: &mut glfw::Window, event: glfw::WindowEvent) {
    //TODO: Handle all events
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            handle.set_should_close(true)
        }
        _ => {}
    }
}
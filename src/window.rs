use bevy_ecs::prelude::*;
use raw_gl_context::{GlContext, GlConfig, Profile};

use winit::{
    event_loop::EventLoop,
    window::WindowBuilder, dpi::{PhysicalSize, PhysicalPosition},
};
use winit::window::CursorGrabMode;

#[derive(Resource)]
pub struct Window {
    handle: winit::window::Window,
    grab_mode: CursorGrabMode,
}

pub struct RenderContext {
    pub handle: GlContext
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str, event_loop: &EventLoop<()>) -> (Window, RenderContext) {
        let window = WindowBuilder::new()
            .with_visible(false) //Make sure to make visible
            .with_title(title.to_string())
            .with_inner_size(PhysicalSize::new(width, height))
            .build(&event_loop).unwrap();

        let context = unsafe { 
            GlContext::create(&window, GlConfig {
                version: (4, 6),
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

        window.set_visible(true);

        let mut our_window = Window { handle: window, grab_mode: CursorGrabMode::None };

        our_window.center_on_display();
        our_window.init_gl();
        
        let our_context = RenderContext { handle: context };

        (our_window, our_context)
    }

    pub fn center_on_display(&mut self) {
        let win_size = self.handle.inner_size();
        let monitor_size = self.handle.primary_monitor().unwrap().size();
        self.handle.set_outer_position(PhysicalPosition::new(
            (monitor_size.width - win_size.width) / 2, 
            (monitor_size.height - win_size.height) / 2));
    }

    pub fn center_of_window(&self) -> PhysicalPosition<u32> {
        let win_size = self.handle.inner_size();
        PhysicalPosition::new(
            win_size.width / 2,
            win_size.height / 2)
    }

    pub fn init_gl(&self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Enable(gl::DEPTH_TEST);
        }
        //TODO: More GL init please
    }


    pub fn aspect_ratio(&self) -> f32 {
        let size = self.handle.inner_size();
        size.width as f32 / size.height as f32
    }

    pub fn cursor_grab(&self) -> CursorGrabMode {
        self.grab_mode
    }

    pub fn set_cursor_grab(&mut self, mut mode: CursorGrabMode) {

        if cfg!(target_os = "macos") && mode == CursorGrabMode::Confined {
            mode = CursorGrabMode::Locked;
        }


        // match mode {
        //     CursorGrabMode::None => self.handle.set_cursor_visible(true),
        //     CursorGrabMode::Confined => self.handle.set_cursor_visible(false),
        //     CursorGrabMode::Locked => panic!(), //Platform incompatibility, always expect Confined and change to Locked on macOS
        // }

        if cfg!(target_os = "macos") {
            mode = CursorGrabMode::Locked;
        }

        let _ = self.handle.set_cursor_grab(mode);
        self.grab_mode = mode;
    }

    pub fn update(&mut self) {
        if self.grab_mode == CursorGrabMode::Confined && self.handle.has_focus() {
            let _ = self.handle.set_cursor_position(self.center_of_window());
        }
    }
}


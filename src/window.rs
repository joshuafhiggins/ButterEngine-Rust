use bevy_ecs::prelude::*;

use sdl2::{Sdl, video::GLContext};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::video::GLProfile;

use crate::resources::{Input, WindowResource};

pub struct Window {
    handle: sdl2::video::Window,
    pub sdl: Sdl,
    gl: GLContext,
    //events: EventPump,
    should_close: bool,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        let gl_attr = video_subsystem.gl_attr();
        // #[cfg(target_os = "macos")]
        // gl_attr.set_context_profile(GLProfile::Compatibility);
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(4, 6);

        let window = video_subsystem.window(title, width, height)
            .opengl()
            .build()
            .unwrap();

        // Unlike the other example above, nobody created a context for your window, so you need to create one.
        let ctx = window.gl_create_context().unwrap(); //GLContext

        // debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        // debug_assert_eq!(gl_attr.context_version(), (4, 3));

        // our_window.handle.make_current();
        // our_window.handle.set_all_polling(true);
        // our_window.set_swap_interval(framerate);

        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        window.gl_make_current(&ctx).unwrap();

        //let event_pump = sdl_context.event_pump().unwrap();

        let mut our_window = Window { handle: window, sdl: sdl_context, gl: ctx, /*events: event_pump,*/ should_close: false };
        our_window.center();
        our_window.init_gl();

        our_window
    }

    pub fn center(&mut self) {
        //let (win_width, win_height) = self.handle.size();
        self.handle.set_position(sdl2::video::WindowPos::Centered, sdl2::video::WindowPos::Centered);
        
        // self.glfw.with_primary_monitor(|_, monitor| {
        //    let vid_mode = monitor.expect("Unable to get the primrary monitor!").get_video_mode().expect("Unable to get the VidMode of the primrary monitor!");
        //    self.handle.set_pos((vid_mode.width as i32 - win_width) / 2, (vid_mode.height as i32 - win_height) / 2);
        // });
    }

    pub fn swap(&self) {
        self.handle.gl_swap_window();
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    // pub fn set_swap_interval(&mut self, interval: i32) {
    //     unsafe {
    //         ffi::glfwSwapInterval(interval as c_int);
    //     }
    // }

    pub fn init_gl(&self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Enable(gl::DEPTH_TEST);
        }
        //TODO: More GL init please
    }

    pub fn aspect_ratio(&self) -> f32 {
        let size = self.handle.drawable_size();
        size.0 as f32 / size.1 as f32
    }

    pub fn exit(&mut self) {
        self.should_close = true;
    }

    // pub fn events(&mut self) -> &mut EventPump {
    //     &mut self.events
    // } 

    pub fn handle_window_event(&mut self, event: Event, world: &mut World) {
        //TODO: Handle all events
        match event {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.exit();
            },
            Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                let mut input = world.get_resource_mut::<Input<Keycode>>().unwrap();
                self.sdl.mouse().set_relative_mouse_mode(true);
                input.set_cursor_enabled(true);
                //TODO: Move to system, have input handle mouse buttons
            },
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                let mut input = world.get_resource_mut::<Input<Keycode>>().unwrap();
                self.sdl.mouse().set_relative_mouse_mode(false);
                input.set_cursor_enabled(false);
                //TODO: Move to system
            },
            Event::KeyDown { keycode: Some(keycode), .. } => {
                let mut input = world.get_resource_mut::<Input<Keycode>>().unwrap();
                input.keyboard_down(keycode);
            },
            Event::KeyUp { keycode: Some(keycode), .. } => {
                let mut input = world.get_resource_mut::<Input<Keycode>>().unwrap();
                input.keyboard_up(keycode);
            },
            Event::MouseMotion { xrel, yrel, .. } => {
                let mut input = world.get_resource_mut::<Input<Keycode>>().unwrap();
                input.dispatch_mouse(xrel as f64, yrel as f64);
            },
            Event::Window { win_event, .. } => {
                match win_event {
                    //Values in the enum may not be the drawable size
                    sdl2::event::WindowEvent::SizeChanged(_, _) => {
                        let mut win_resource = world.get_resource_mut::<WindowResource>().unwrap();
                        win_resource.set_ratio(self);
                        let new_size = self.handle.drawable_size();
                        unsafe {
                            gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32);
                        }
                    },
                    _ => {},
                }
            },
            _ => {}
        }
    }
}
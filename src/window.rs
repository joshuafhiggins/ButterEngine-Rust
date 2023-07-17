use bevy_ecs::prelude::*;
use raw_gl_context::{GlContext, GlConfig, Profile};

use crate::resources::*;

use winit::{
    event::{Event, WindowEvent, DeviceEvent, RawKeyEvent, ElementState, KeyEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder, dpi::{Size, PhysicalSize, PhysicalPosition}, keyboard::KeyCode,
};

#[derive(Resource)]
pub struct Window {
    handle: winit::window::Window
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

        let mut our_window = Window { handle: window };

        our_window.center();
        our_window.init_gl();
        
        let our_context = RenderContext { handle: context };

        (our_window, our_context)
    }

    pub fn center(&mut self) {
        let win_size = self.handle.inner_size();
        let monitor_size = self.handle.primary_monitor().unwrap().size();
        self.handle.set_outer_position(PhysicalPosition::new(
            (monitor_size.width - win_size.width) / 2, 
            (monitor_size.height - win_size.height) / 2));
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

    pub fn handle_window_event(&mut self, world: &mut World, event: Event<()>, control_flow: &mut ControlFlow) {
        match event { //TODO: Handle all events
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::KeyboardInput { event: KeyEvent { physical_key: KeyCode::Escape, state: ElementState::Pressed, .. }, .. } => {
                        control_flow.set_exit();
                    },
                    WindowEvent::KeyboardInput { event: KeyEvent { physical_key, state, .. }, .. } => {
                        let mut input = world.get_resource_mut::<Input<KeyCode>>().unwrap();
                        input.dispatch_keyboard(physical_key, state);
                    },
                    WindowEvent::Resized(size) => {
                        unsafe {
                            gl::Viewport(0, 0, size.width as i32, size.height as i32);
                        }
                    }
                    // DeviceEvent::Button { button, state } => {
                    //     let mut input = world.get_resource_mut::<Input<KeyCode>>().unwrap();
                    //     input.dispatch_keyboard(physical_key, state);
                    // }
                    
                    _ => {}
                }
            },
            Event::DeviceEvent { event, .. } => {
                match event {
                    DeviceEvent::MouseMotion { delta } => {
                        let mut input = world.get_resource_mut::<Input<KeyCode>>().unwrap();
                        input.dispatch_mouse_motion(delta.0, delta.1);
                    },
                    _ => {}
                }
            },

            // glfw::WindowEvent::MouseButton(glfw::MouseButton::Button1, glfw::Action::Press, _) => {
            //     let mut input = world.get_resource_mut::<Input<glfw::Key>>().unwrap();
            //     self.handle.set_cursor_mode(glfw::CursorMode::Disabled);
            //     input.set_cursor_mode(glfw::CursorMode::Disabled);
            // }
            // glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) => {
            //     let mut input = world.get_resource_mut::<Input<glfw::Key>>().unwrap();
            //     self.handle.set_cursor_mode(glfw::CursorMode::Normal);
            //     input.set_cursor_mode(glfw::CursorMode::Normal);
            // }
            _ => {}
        }
    }
}


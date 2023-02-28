mod window;
mod bufferobject;
mod vertex;
use window::Window;

fn main() {
    //TODO: Load settings from text file
    let mut window = Window::new(1280, 720, "Butter Engine", glfw::WindowMode::Windowed, glfw::SwapInterval::Sync(1));
    window.center();
    window.init_gl();

    while !window.should_close() {
        //TODO: Render
        //TODO: ecs/physics/sound/ui/engine updates


        window.process_events();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        window.poll_events();
    }
}



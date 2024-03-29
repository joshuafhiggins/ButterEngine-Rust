use crate::{components::*, resources::*, settings::Settings, mesh::Mesh, renderer::GPUObject, window::Window};
use bevy_ecs::prelude::*;
use winit::{keyboard::KeyCode, window::CursorGrabMode};
use winit::event::MouseButton;

const CAMERA_SPEED: f32 = 1.0; // adjust accordingly
const SENSITIVITY: f32 = 0.1;

pub fn move_camera(
    mut query: Query<(&mut Position, &mut Rotation, &mut Camera)>,
    input: Res<Input>,
    mut window: ResMut<Window>,
    time: Res<Time>,
) {
    for (mut position, mut direction, mut camera) in &mut query {
        if input.mouse_just_pressed(MouseButton::Left) {
            window.set_cursor_grab(CursorGrabMode::Confined);
        }
        if input.mouse_just_pressed(MouseButton::Right) {
            window.set_cursor_grab(CursorGrabMode::None);
        }

        if window.cursor_grab() == CursorGrabMode::Confined {
            let move_factor = CAMERA_SPEED * time.delta_seconds();
            if input.keyboard_pressed(KeyCode::KeyW) {
                position.d += move_factor * camera.front;
            }
            if input.keyboard_pressed(KeyCode::KeyS) {
                position.d -= move_factor * camera.front;
            }
            if input.keyboard_pressed(KeyCode::KeyA) {
                position.d -= camera.front.cross(camera.up).normalize() * move_factor;
            }
            if input.keyboard_pressed(KeyCode::KeyD) {
                position.d += camera.front.cross(camera.up).normalize() * move_factor;
            }

            let mut xoffset = input.delta_xpos();
            let mut yoffset = -input.delta_ypos();

            if camera.first_mouse {
                xoffset = 0.0;
                yoffset = 0.0;
                camera.first_mouse = false;
            }

            xoffset *= SENSITIVITY as f64;
            yoffset *= SENSITIVITY as f64;

            camera.yaw += xoffset;
            camera.pitch += yoffset;

            if camera.pitch > 89.0 {
                camera.pitch = 89.0;
            }
            if camera.pitch < -89.0 {
                camera.pitch = -89.0;
            }

            direction.d.x =
                (libm::cos(camera.yaw.to_radians()) * libm::cos(camera.pitch.to_radians())) as f32;
            direction.d.y = libm::sin(camera.pitch.to_radians()) as f32;
            direction.d.z =
                (libm::sin(camera.yaw.to_radians()) * libm::cos(camera.pitch.to_radians())) as f32;
            camera.front = direction.d.normalize();
        }
        camera.view = glam::Mat4::look_at_rh(position.d, position.d + camera.front, camera.up);

    }
}

pub fn update_wireframe(input: Res<Input>, mut settings: ResMut<Settings>) {
    if input.keyboard_just_pressed(KeyCode::F5) {
        crate::renderer::toggle_wireframe(&mut settings.is_wireframe);
    }
}

pub fn update_projection(mut query: Query<&mut Camera>, window: Res<Window>, settings: Res<Settings>) {
    if window.is_changed() || settings.is_changed() {
        for mut camera in &mut query {
            camera.set_projection(settings.fov, window.aspect_ratio(), 0.01, 100.0);
        }
    }
}

pub fn render_scene(mut query_mesh: Query<&Mesh>, mut query_camera: Query<&Camera>, assets: Res<AssetPool>) {
    for camera in &mut query_camera {
        for mesh in &mut query_mesh {
            //TODO: Support multiple textures
            let material = assets.get_material(&mesh.material).unwrap();
            let shader = assets.get_shader(&material.shader).unwrap();
            let texture = assets.get_texture(&material.textures.get(0).unwrap().0).unwrap();
    
            shader.bind();
            shader.set_uniform_4x4f("camMatrix".to_string(), None, &camera.get_calculation());
            texture.bind();
    
            mesh.render();
    
            texture.unbind();
            shader.unbind();
        }
    }
}
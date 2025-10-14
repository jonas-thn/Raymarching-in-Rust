use crate::camera::Camera;
use crate::scene::{self, get_normal, scene_sdf};
use glam::Vec3;
use pixels::Pixels;
use std::sync::Arc;
use std::time::Instant;
use winit::{
    event::{DeviceEvent, ElementState, KeyEvent, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window},
};

const MAX_STEPS: u32 = 32;
const MAX_SHADOW_STEPS: u32 = 16;
const HIT_THRESHOLD: f32 = 0.001;

pub struct App<'a> {
    pub window: Arc<Window>,
    pixels: Pixels<'a>,
    width: u32,
    height: u32,
    render_scale: f32,
    camera: Camera,
    last_frame_time: Instant,
    keys: Keys,
    time: f32,
}

#[derive(Default)]
struct Keys {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
}

impl<'a> App<'a> {
    pub fn new(
        window: Arc<Window>,
        pixels: Pixels<'a>,
        width: u32,
        height: u32,
        render_scale: f32,
    ) -> Self {
        window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
        window.set_cursor_visible(false);

        Self {
            window,
            pixels,
            width,
            height,
            render_scale,
            camera: Camera::new(Vec3::new(0.0, 0.0, -4.0)),
            last_frame_time: Instant::now(),
            keys: Keys::default(),
            time: 0.0,
        }
    }

    pub fn handle_window_event(&mut self, event: WindowEvent, elwt: &ActiveEventLoop) -> bool {
        match event {
            WindowEvent::CloseRequested => {
                return false;
            }
            WindowEvent::Resized(size) => {
                let new_render_width = (size.width as f32 * self.render_scale).max(1.0) as u32;
                let new_render_height = (size.height as f32 * self.render_scale).max(1.0) as u32;

                self.width = size.width;
                self.height = size.height;

                if let Err(err) = self.pixels.resize_surface(size.width, size.height) {
                    log::error!("Resize Surface Error: {err}");
                    elwt.exit();
                }

                if let Err(err) = self
                    .pixels
                    .resize_buffer(new_render_width, new_render_height)
                {
                    log::error!("Resize Buffer failed: {err}");
                    elwt.exit();
                }

                self.width = new_render_width;
                self.height = new_render_height;

                self.window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    let is_pressed = event.state == ElementState::Pressed;
                    match key {
                        KeyCode::KeyW => self.keys.w = is_pressed,
                        KeyCode::KeyA => self.keys.a = is_pressed,
                        KeyCode::KeyS => self.keys.s = is_pressed,
                        KeyCode::KeyD => self.keys.d = is_pressed,

                        KeyCode::Escape => {
                            if is_pressed {
                                elwt.exit();
                                self.window.set_cursor_grab(CursorGrabMode::None).unwrap();
                                self.window.set_cursor_visible(true);
                            }
                        }
                        _ => {}
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                self.update();
                self.draw();

                if let Err(err) = self.pixels.render() {
                    log::error!("Render Error: {err}");
                    elwt.exit();
                }
            }

            _ => {}
        }

        true
    }

    pub fn handle_device_event(&mut self, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.camera.update_rotation(delta.0 as f32, delta.1 as f32);
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let dt = (now - self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;
        self.time += dt;

        if self.keys.w {
            self.camera.move_forward(dt);
        }
        if self.keys.s {
            self.camera.move_backward(dt);
        }
        if self.keys.a {
            self.camera.move_left(dt);
        }
        if self.keys.d {
            self.camera.move_right(dt);
        }
    }

    fn draw(&mut self) {
        let aspect_ratio = self.width as f32 / self.height as f32;

        let camera_pos = self.camera.position;
        let focal_length = self.camera.focal_length;

        // from surface to light source !!!
        // let light_dir = Vec3::new(0.5, 0.5, -1.0).normalize();
        let light_dir = Vec3::new(0.0, 0.5, -1.0).normalize();

        let mut colors: Vec<[u8; 4]> = vec![[0; 4]; (self.width * self.height) as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                let u = (x as f32 / self.width as f32) - 0.5;
                let v = (y as f32 / self.height as f32) - 0.5;

                let ray_dir = self.camera.calculate_ray_dir(u, v, aspect_ratio);

                let hit_info = self.raymarch(camera_pos, ray_dir);

                let final_color = if let Some((point, color)) = hit_info {
                    self.get_color_for_hit(point, light_dir, color)
                } else {
                    [0x00, 0x00, 0x00, 0xFF]
                };

                let index = (y as usize * self.width as usize) + x as usize;
                colors[index] = final_color;
            }
        }

        let frame = self.pixels.frame_mut();

        for (i, color) in colors.iter().enumerate() {
            let idx = i * 4;
            frame[idx..idx + 4].copy_from_slice(color);
        }
    }

    fn raymarch(&self, ray_origin: Vec3, ray_dir: Vec3) -> Option<(Vec3, Vec3)> {
        let mut current_pos = ray_origin;
        for _ in 0..MAX_STEPS {
            let (dist_to_scene, color) = scene_sdf(current_pos, self.time);
            if dist_to_scene < HIT_THRESHOLD {
                return Some((current_pos, color));
            }
            current_pos += ray_dir * dist_to_scene;
            if current_pos.distance(ray_origin) > 100.0 {
                break;
            }
        }
        None
    }

    fn in_shadow(&self, point: Vec3, direction: Vec3) -> bool {
        let mut current_pos = point + get_normal(point, self.time) * 0.01;

        for _ in 0..MAX_SHADOW_STEPS {
            let dist_to_scene = scene_sdf(current_pos, self.time).0;

            if dist_to_scene < HIT_THRESHOLD {
                return true;
            }

            current_pos += direction * dist_to_scene;
        }

        false
    }

    fn get_color_for_hit(&self, hit_point: Vec3, light_dir: Vec3, base_color: Vec3) -> [u8; 4] {
        let normal = get_normal(hit_point, self.time);

        let diffuse_intensity = normal.dot(light_dir).max(0.0);

        let shadow_factor = if self.in_shadow(hit_point, light_dir) {
            0.05
        } else {
            1.0
        };

        let ambient_light = 0.05;
        let final_intensity = diffuse_intensity * shadow_factor + ambient_light;
        let mut color_vec = base_color * final_intensity;

        color_vec = color_vec.powf(0.5);

        [
            (color_vec.x.clamp(0.0, 1.0) * 255.0) as u8,
            (color_vec.y.clamp(0.0, 1.0) * 255.0) as u8,
            (color_vec.z.clamp(0.0, 1.0) * 255.0) as u8,
            0xFF,
        ]
    }
}

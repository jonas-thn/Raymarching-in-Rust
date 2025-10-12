use crate::scene::{self, get_normal, scene_sdf};
use glam::Vec3;
use pixels::Pixels;
use std::sync::Arc;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

pub struct App<'a> {
    window: Arc<Window>,
    pixels: Pixels<'a>,
    width: u32,
    height: u32,
}

impl<'a> App<'a> {
    pub fn new(window: Arc<Window>, pixels: Pixels<'a>, width: u32, height: u32) -> Self {
        Self {
            window,
            pixels,
            width,
            height,
        }
    }

    pub fn handle_event(&mut self, event: WindowEvent, elwt: &ActiveEventLoop) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window closed!");
                elwt.exit();
            }
            WindowEvent::Resized(size) => {
                self.width = size.width;
                self.height = size.height;

                if let Err(err) = self.pixels.resize_surface(size.width, size.height) {
                    log::error!("Resize Surface Error: {err}");
                    elwt.exit();
                }

                if let Err(err) = self.pixels.resize_buffer(size.width, size.height) {
                    log::error!("Resize Buffer failed: {err}");
                    elwt.exit();
                    return;
                }

                self.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                self.draw();

                if let Err(err) = self.pixels.render() {
                    log::error!("Render Error: {err}");
                    elwt.exit();
                }
            }

            _ => {
                self.window.request_redraw();
            }
        }
    }

    fn draw(&mut self) {
        let aspect_ratio = self.width as f32 / self.height as f32;

        let camera_pos = Vec3::new(0.0, 0.0, -3.0);
        let focal_length = 0.5;

        // from surface to light source !!!
        // let light_dir = Vec3::new(0.5, 0.5, -1.0).normalize();
        let light_dir = Vec3::new(0.0, 0.5, -1.0).normalize();


        let mut colors: Vec<[u8; 4]> = vec![[0; 4]; (self.width * self.height) as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                let u = (x as f32 / self.width as f32) - 0.5;
                let v = (y as f32 / self.height as f32) - 0.5;
                let u_corrected = u * aspect_ratio;
                let ray_dir = Vec3::new(u_corrected, -v, focal_length).normalize();

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
        for _ in 0..100 {
            let (dist_to_scene, color) = scene_sdf(current_pos);
            if dist_to_scene < 0.001 {
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
        let mut current_pos = point + get_normal(point) * 0.01;

        for _ in 0..50 {
            let dist_to_scene = scene_sdf(current_pos).0;

            if dist_to_scene < 0.001 {
                return true;
            }

            current_pos += direction * dist_to_scene;
        }

        false
    }

    fn get_color_for_hit(&self, hit_point: Vec3, light_dir: Vec3, base_color: Vec3) -> [u8; 4] {
        let normal = get_normal(hit_point);

        let diffuse_intensity = normal.dot(light_dir).max(0.0);

        let shadow_factor = if self.in_shadow(hit_point, light_dir) { 0.1 } else { 1.0 };   

        let ambient_light = 0.1;
        let final_intensity = diffuse_intensity * shadow_factor + ambient_light;
        let mut color_vec = base_color * final_intensity;

        color_vec = color_vec.powf(1.0 / 2.2);

        [
            (color_vec.x.clamp(0.0, 1.0) * 255.0) as u8,
            (color_vec.y.clamp(0.0, 1.0) * 255.0) as u8,
            (color_vec.z.clamp(0.0, 1.0) * 255.0) as u8,
            0xFF,
        ]
    }
}

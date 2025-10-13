mod app;
mod camera;
mod scene;

use app::App;
use glam::Vec3;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const RENDER_SCALE: f32 = 0.25;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    let mut app: Option<App> = None;

    event_loop
        .run(move |event, window_target| match event {
            Event::Resumed => {
                let window = Arc::new(
                    window_target
                        .create_window(
                            Window::default_attributes()
                                .with_title("Raymarching in Rust")
                                .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
                                .with_min_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
                        )
                        .unwrap(),
                );

                let pixels = {
                    let window_size = window.inner_size();
                    let surface_texture =
                        SurfaceTexture::new(window_size.width, window_size.height, window.clone());

                    let render_width = (window_size.width as f32 * RENDER_SCALE) as u32;
                    let render_height = (window_size.height as f32 * RENDER_SCALE) as u32;
                    
                    Pixels::new(render_width, render_height, surface_texture).unwrap()
                };

                let initial_render_width = (WIDTH as f32 * RENDER_SCALE) as u32;
                let initial_render_height = (HEIGHT as f32 * RENDER_SCALE) as u32;

                app = Some(App::new(
                    window,
                    pixels,
                    initial_render_width,
                    initial_render_height,
                    RENDER_SCALE
                ));
            }

            Event::WindowEvent { window_id, event } => {
                if let Some(app) = app.as_mut() {
                    if !app.handle_event(event, window_target) {
                        window_target.exit();
                    }
                }
            }

            Event::AboutToWait => {
                if let Some(app) = app.as_mut() {
                    app.window.request_redraw();
                }
            }

            _ => (),
        })
        .unwrap();
}

mod app;
mod scene;

use app::App;

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
                    let surface_texture = SurfaceTexture::new(
                        window_size.width,
                        window_size.height,
                        window.clone(),
                    );
                    Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
                };

                app = Some(App::new(window, pixels, WIDTH, HEIGHT));
            }

            Event::WindowEvent {
                window_id: _,
                event,
            } => {
                if let Some(app) = app.as_mut() {
                    app.handle_event(event, window_target);
                }
            }

            _ => (),
        })
        .unwrap();
}

use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), pixels::Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let mut window: Option<Arc<Window>> = None;
    let mut pixels: Option<Pixels> = None;

    event_loop
        .run(move |event, window_target| {
            match event {
                Event::Resumed => {
                    let new_window = Arc::new(
                        window_target
                            .create_window(
                Window::default_attributes()
                                    .with_title("Rust Raymarcher")
                                    .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
                                    .with_min_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
                            )
                            .unwrap(),
                    );

                    let new_pixels = {
                        let window_size = new_window.inner_size();
                        let surface_texture = SurfaceTexture::new(
                            window_size.width,
                            window_size.height,
                            new_window.clone(),
                        );
                        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
                    };

                    window = Some(new_window);
                    pixels = Some(new_pixels);
                }

                Event::WindowEvent { event, .. } => {
                    if let (Some(win), Some(px)) = (window.as_ref(), pixels.as_mut()) {
                        match event {
                            WindowEvent::CloseRequested => {
                                println!("Das Fenster wird geschlossen.");
                                window_target.exit();
                            }

                            WindowEvent::Resized(size) => {
                                if let Err(err) = px.resize_surface(size.width, size.height) {
                                    log::error!("pixels.resize_surface() failed: {err}");
                                    window_target.exit();
                                }
                            }

                            WindowEvent::RedrawRequested => {
                                //DRAW

                                if let Err(err) = px.render() {
                                    log::error!("pixels.render() failed: {err}");
                                    window_target.exit();
                                }
                            }

                            _ => {
                                win.request_redraw();
                            }
                        }
                    }
                }

                Event::LoopExiting => {
                    println!("Event-Schleife wird beendet.");
                }

                _ => (),
            }
        })
        .unwrap();

    Ok(())
}

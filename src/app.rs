use pixels::Pixels;
use std::sync::Arc;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct App<'a> {
    window: Arc<Window>,
    pixels: Pixels<'a>,
}

impl<'a> App<'a> {
    pub fn new(window: Arc<Window>, pixels: Pixels<'a>) -> Self {
        Self { window, pixels }
    }

    pub fn handle_event(&mut self, event: WindowEvent, elwt: &ActiveEventLoop) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window closed!");
                elwt.exit();
            }
            WindowEvent::Resized(size) => {
                if let Err(err) = self.pixels.resize_surface(size.width, size.height) {
                    log::error!("Resize Error: {err}");
                    elwt.exit();
                }
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
        let frame = self.pixels.frame_mut();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as u32;
            let y = (i / WIDTH as usize) as u32;

            let r = (x as f32 / WIDTH as f32 * 255.0) as u8;
            let g = (y as f32 / HEIGHT as f32 * 255.0) as u8;
            let b = 128;

            let rgba = [r, g, b, 0xFF];
            pixel.copy_from_slice(&rgba);
        }
    }
}

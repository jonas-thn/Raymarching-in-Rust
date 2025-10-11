use pixels::Pixels;
use std::sync::Arc;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

pub struct App<'a> {
    window: Arc<Window>,
    pixels: Pixels<'a>,
    width: u32,
    height: u32
}

impl<'a> App<'a> {
    pub fn new(window: Arc<Window>, pixels: Pixels<'a>, width: u32, height: u32) -> Self {
        Self { window, pixels, width, height }
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
        //buffer of bytes -> RGBA = 4 bytes per pixel
        let frame = self.pixels.frame_mut();
        
        let aspect_ratio = self.width as f32 / self.height as f32;

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as u32; //0-800
            let y = (i / self.width as usize) as u32; //0-600

            let u = (x as f32 / self.width as f32) - 0.5; //-0.5 to 0.5
            let v = (y as f32 / self.height as f32) - 0.5; //-0.5 to 0.5

            let u_corrected = u * aspect_ratio; //x axis bigger or smaller based on aspect ratio
            
            let r = ((u_corrected + 0.5) * 255.0) as u8;
            let g = ((v + 0.5) * 255.0) as u8;
            let b = 128;

            let rgba = [r, g, b, 0xFF];
            pixel.copy_from_slice(&rgba);
        }
    }
}

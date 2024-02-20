use std::sync::mpsc::Receiver;

use blue_box::{types::desc::PixelIntensity, utils::colors::color_palette};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

pub struct DisplayFractal {
    width: u16,
    height: u16,
}

impl DisplayFractal {
    pub fn new(width: u16, height: u16) -> Self {
        DisplayFractal { width, height }
    }

    pub fn start(&self, rx: Receiver<Vec<PixelIntensity>>) -> Result<(), Error> {
        let event_loop = EventLoop::new();
        let mut _input = WinitInputHelper::new();

        let window = {
            let size = LogicalSize::new(self.width as f64, self.height as f64);
            WindowBuilder::new()
                .with_title("Fractal")
                .with_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(self.width as u32, self.height as u32, surface_texture)?
        };

        event_loop.run(move |event, _, control_flow| {
            if let Ok(data_intensity) = rx.try_recv() {
                let colors = DisplayFractal::from_count_to_colors(data_intensity);
                DisplayFractal::draw(pixels.frame_mut(), colors);
                let _ = pixels.render();
            }

            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                if let Err(_) = pixels.render() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
        });
    }

    fn draw(frame: &mut [u8], colors: Vec<(u8, u8, u8)>) {
        // INFO: After the 50_000 th pixels start to juste show one color
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let (red, green, blue) = colors[i % colors.len()];
            let rgba = [red, green, blue, 0xff];
            pixel.copy_from_slice(&rgba);
        }
    }

    fn from_count_to_colors(pis: Vec<PixelIntensity>) -> Vec<(u8, u8, u8)> {
        let mut rep: Vec<(u8, u8, u8)> = vec![];
        for pi in pis {
            rep.push(color_palette(pi.count));
        }
        rep
    }
}

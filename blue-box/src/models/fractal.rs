use crate::types::{
    desc::PixelData,
    fractal_type::{CalcFractal, FractalDescriptor},
    protocols::{FragmentResult, FragmentTask},
};

#[derive(Debug)]
pub struct Fractal {}

impl Fractal {
    pub fn run(fragment_task: &FragmentTask, data: &mut Vec<u8>) -> FragmentResult {
        let pixels = PixelData {
            offset: fragment_task.id.count,
            count: fragment_task.resolution.nx as u32 * fragment_task.resolution.ny as u32,
        };

        let fragment_result: FragmentResult = FragmentResult::new(
            fragment_task.id.clone(),
            fragment_task.resolution.clone(),
            fragment_task.range.clone(),
            pixels,
        );

        match &fragment_task.fractal {
            FractalDescriptor::Julia(julia) => julia.make_image(&fragment_task, data),
            FractalDescriptor::Mandelbrot(mandelbrot) => {
                mandelbrot.make_image(&fragment_task, data)
            }
            FractalDescriptor::IteratedSinZ(iter) => iter.make_image(&fragment_task, data),
            FractalDescriptor::NewtonRaphsonZ3(newton) => newton.make_image(&fragment_task, data),
            FractalDescriptor::NewtonRaphsonZ4(newton) => newton.make_image(&fragment_task, data),
        };

        fragment_result
    }
}

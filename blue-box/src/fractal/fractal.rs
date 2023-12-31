use std::{io::Write, fmt::Pointer};

use log::{error, debug};

use crate::network::communication_types::{FragmentResult, FragmentTask, Complex, Resolution, Point};

use super::fractal_types::{FractalDescriptor, JuliaDescriptor};

impl JuliaDescriptor {
    pub fn run(&self, fragment_result: &mut FragmentResult, data: &mut Vec<u8>, max_iteration: &u32) {

        self.make_image(
            &fragment_result.resolution,
            &fragment_result.range.max,
            &fragment_result.range.min,
            max_iteration, 
            data)

    }

    fn make_image(&self, resolution: &Resolution, max: &Point, min: &Point, max_iteration: &u32, data: &mut Vec<u8>) {
        for offset in 0..(resolution.nx * resolution.ny) {
            let x = offset % resolution.nx;
            let y = offset / resolution.nx;

            let mapped_x = min.x + (x as f64 / resolution.nx as f64) * (max.x - min.x);
            let mapped_y = min.y + (y as f64 / resolution.ny as f64) * (max.y - min.y);

            let (zn , count) = self.make_pixel(mapped_x, mapped_y, max_iteration);

            debug!("zn -> {zn}");
            if let Err(_) = data.write_all(&zn.to_be_bytes()){
                error!("Can't add to data")
            }
            debug!("count -> {count}");
            if let Err(_) = data.write_all(&count.to_be_bytes()){
                error!("Can't add to data")
            }
        }
    }

    fn make_pixel(&self, x: f64, y: f64, max_iteration: &u32)-> (f32, f32){

        let mut z = Complex::new(x, y);

        let mut i = 0;
        while i < *max_iteration && z.norm() < self.divergence_threshold_square {
            // debug!("z -> {z:?}, c -> {:?}", self.c);
            z = z * z + self.c;
            i += 1;
        }
        (z.norm() as f32, (i as f32 / *max_iteration as f32))
    }
}

#[derive(Debug)]
pub struct Fractal {}

impl Fractal {
    pub fn run(
        fragment_task: &FragmentTask,
        fragment_result: &mut FragmentResult,
        data: &mut Vec<u8>,
    ) {
        match &fragment_task.fractal {
            FractalDescriptor::Julia(julia) => julia.run(fragment_result, data, &(fragment_task.max_iteration as u32)),
            FractalDescriptor::Mandelbrot(mandelbrot) => todo!(),
            FractalDescriptor::IteratedSinZ(iter) => todo!(),
            FractalDescriptor::NewtonRaphsonZ3(newton) => todo!(),
            FractalDescriptor::NewtonRaphsonZ4(newton) => todo!(),
        }
    }
}

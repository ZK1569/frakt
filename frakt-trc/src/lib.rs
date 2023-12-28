mod complex;
mod complex_test;

use complex::Complex;
use network::models::commmunication::{FragmentResult};

#[derive(Debug)]
pub struct Fractal ;

impl Fractal {

    pub fn calculate_iterations (fragment_result: &mut FragmentResult) -> (){



    }

    // fn calc_one_pixel(re: f32, im: f32, max_iteration: usize)-> (f32, f32){
    //
    //     let c = Complex::new(re, im);
    //     let mut z = Complex::new(0_f32, 0_f32);
    //
    //     let mut nbr_iteration = 0;
    //     while nbr_iteration <= max_iteration && z.norm() < 2.0 {
    //         z = z * z + c;
    //         nbr_iteration += 1;
    //     }
    //
    //     nbr_iteration
    // }

}

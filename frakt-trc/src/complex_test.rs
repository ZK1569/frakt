#![cfg(test)]

use crate::complex::Complex;

#[test]
fn test_add() {
    let _5_4i = Complex::new(5_f32, 4_f32);
    let _7_2i = Complex::new(7_f32, 2_f32);

    let _12_6i = Complex::new(12_f32, 6_f32);

    assert_eq!(_5_4i + _7_2i, _12_6i);
}

#[test]
fn test_mul() {
    let _3_2i = Complex::new(3_f32, 2_f32);
    let _1_7i = Complex::new(1_f32, 7_f32);

    let _1_1i = Complex::new(1_f32, 1_f32);
    let _1_1i_2 = Complex::new(1_f32, 1_f32);

    let __11_23i = Complex::new(-11_f32, 23_f32);
    let _0_2i = Complex::new(0_f32, 2_f32);

    assert_eq!(_3_2i * _1_7i, __11_23i);
    assert_eq!(_1_1i * _1_1i_2, _0_2i);
}

#[test]
fn cal_norm() {
    let _3_2i = Complex::new(3_f32, 2_f32);
    let _1_7i = Complex::new(1_f32, 7_f32);
    let __11_23i = Complex::new(-11_f32, 23_f32);

    assert_eq!(3.6055512, _3_2i.norm());
    assert_eq!(7.071068, _1_7i.norm());
    assert_eq!(25.495098, __11_23i.norm());

}

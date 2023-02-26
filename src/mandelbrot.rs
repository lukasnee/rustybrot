#[path = "complex.rs"]
pub mod complex;
use complex::Complex;

const ESCAPE_MAGNITUDE: f64 = 2.0;

pub fn get_iterations(c: &Complex, max_iterations: u64) -> u64 {
    let mut z = Complex::new(0.0, 0.0);
    for x in 1..max_iterations {
        z = z.mul(&z).add(&c);
        if z.modulus() > ESCAPE_MAGNITUDE {
            return x;
        }
    }
    max_iterations
}

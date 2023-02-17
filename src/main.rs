mod complex;
use complex::Complex;

const MAX_ITERATIONS: u64 = 30;
const SCREEN_SIZE: u64 = 400;
const VIEW_SPAN_X: f64 = 4.0;
const VIEW_SPAN_Y: f64 = 4.0;

// const PALLETE: &str = "%#*-.";

fn get_iterations(c: &Complex) -> u64 {
    let mut z_curr = Complex {
        real: 0.0,
        imag: 0.0,
    };
    for x in 1..MAX_ITERATIONS {
        z_curr = z_curr.mul(&z_curr).add(&c);
        if z_curr.modulus() > 2.0 {
            return x;
        }
    }
    MAX_ITERATIONS
}

fn map(
    input_value: u64,
    input_min: u64,
    input_max: u64,
    output_min: u64,
    output_max: u64,
) -> u64 {
    (input_value - input_min) * (output_max - output_min)
        / (input_max - input_min)
        + output_min
}

fn print_line(x_steps: u64, imag: f64) {
    let pallete: Vec<char> = vec!['%', '#', '*', '-', '.']; // TODO optimze

    let step_size = VIEW_SPAN_X / (x_steps as f64);
    for step in 0..(x_steps + 1) {
        let c = Complex {
            real: ((step as f64) * step_size) - (VIEW_SPAN_X / 2.0),
            imag,
        };
        let iterations = get_iterations(&c);
        let color =
            map(iterations, 1, MAX_ITERATIONS, 0, (pallete.len() as u64) - 1)
                as usize;
        print!("{}", pallete[color]);
    }
    println!();
}

fn main() {
    let step_size = VIEW_SPAN_Y / (SCREEN_SIZE as f64);
    for step in 0..(SCREEN_SIZE + 1) {
        let imag = ((step as f64) * step_size) - (VIEW_SPAN_Y / 2.0);
        // println!("{}", imag);
        print_line(SCREEN_SIZE * 2, imag);
    }
}

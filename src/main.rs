mod complex;
use complex::Complex;

fn get_iterations(c: &Complex, max_iterations_last_avg: u64) -> u64 {
    let mut z_curr = Complex {
        real: 0.0,
        imag: 0.0,
    };
    for x in 1..max_iterations_last_avg {
        z_curr = z_curr.mul(&z_curr).add(&c);
        if z_curr.modulus() > 4.0 {
            return x;
        }
    }
    max_iterations_last_avg
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

fn print_line(
    real_offset: f64,
    real_span: f64,
    real_steps: u64,
    imag: f64,
    max_iterations_last_avg: u64,
) {
    let pallete: Vec<char> = vec!['@', '#', '%', '+', '*', '-', '.', '`', ' ']; // TODO optimize

    let step_size = real_span / (real_steps as f64);
    for step in 0..(real_steps + 1) {
        let c = Complex {
            real: ((step as f64) * step_size) - (real_span / 2.0) + real_offset,
            imag,
        };
        let iterations = get_iterations(&c, max_iterations_last_avg);
        let color = map(
            iterations,
            1,
            max_iterations_last_avg,
            0,
            (pallete.len() as u64) - 1,
        ) as usize;
        print!("{}", pallete[pallete.len() - 1 - color]);
    }
    // println!();
}

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    // let mut stdout = stdout();

    let mut real_offset: f64 = 0.0;
    let mut imag_offset: f64 = 0.0;
    let mut view_size: f64 = 4.0;
    let mut max_iterations_last_avg: u64 = 100;
    const max_iterations: u64 = 30;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => view_size += (view_size / 10.0),
            Key::Char('w') => imag_offset -= (view_size / 10.0),
            Key::Char('e') => view_size -= (view_size / 10.0),
            Key::Char('a') => real_offset -= (view_size / 10.0),
            Key::Char('s') => imag_offset += (view_size / 10.0),
            Key::Char('d') => real_offset += (view_size / 10.0),
            Key::Char('1') => {
                max_iterations_last_avg +=
                    (((max_iterations_last_avg as f64) / 10.0) as u64)
            }
            Key::Char('2') => {
                max_iterations_last_avg -=
                    (((max_iterations_last_avg as f64) / 10.0) as u64)
            }

            Key::Ctrl('z') => break,
            _ => (),
        }

        let (terminal_width, terminal_height) =
            termion::terminal_size().unwrap();
        let aspect_ratio = (terminal_width as f64) / (terminal_height as f64);

        let real_span = view_size;
        let imag_span = view_size / aspect_ratio;

        let step_size = imag_span / ((terminal_height / 2) as f64);
        for step in 0..((terminal_height) + 1) {
            let imag =
                ((step as f64) * step_size) - (imag_span / 2.0) + imag_offset;
            // println!("{}", imag);
            print_line(
                real_offset,
                real_span,
                (terminal_width - 1).into(),
                imag,
                max_iterations_last_avg,
            );
        }

        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        write!(stdout, "{}", termion::clear::All).unwrap();
    }
}

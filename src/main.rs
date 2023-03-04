mod camera;
use camera::Camera;
mod stats;
use stats::Stats;
mod mandelbrot;
use mandelbrot::complex::Complex;
#[macro_use]
mod map;
use std::io::{stdin, stdout, Stdout, Write};
use std::vec;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

struct Screen<'a> {
    terminal_width: u16,
    terminal_height: u16,
    terminal_glyph_ratio: f64,
    vram: &'a mut Vec<u64>,
}

fn render_mandelbrot(
    screen: &mut Screen,
    camera: &Camera,
    stats: &mut Stats,
    max_iterations: u64,
) {
    let real_step_size = camera.real_span / (screen.terminal_width as f64);
    let imag_step_size = camera.imag_span / (screen.terminal_height as f64);
    for imag_step in 0..(screen.terminal_height) as usize {
        let imag = (camera.imag_offset - (camera.imag_span / 2.0))
            + ((imag_step as f64) * imag_step_size);
        for real_step in 0..(screen.terminal_width) as usize {
            let real = (camera.real_offset - (camera.real_span / 2.0))
                + ((real_step as f64) * real_step_size);
            let c = Complex { real, imag };
            let pixel_index =
                (imag_step * screen.terminal_height as usize) + real_step;
            screen.vram[pixel_index] =
                mandelbrot::get_iterations(&c, max_iterations);
            stats.update(screen.vram[pixel_index]);
        }
    }
}

fn draw_mandelbrot(
    screen: &Screen,
    stdout: &mut RawTerminal<Stdout>,
    stats: &Stats,
) {
    write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    // let pallete = String::from("$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ");
    let pallete = String::from("@#%+*-.` ");
    for imag_step in 0..(screen.terminal_height) as usize {
        // write!(stdout, "{}", termion::cursor::Goto(1, imag_step as u16)).unwrap();
        for real_step in 0..(screen.terminal_width) as usize {
            let pixel_index =
                (imag_step * screen.terminal_height as usize) + real_step;
            let color = map!(
                screen.vram[pixel_index],
                stats.get_min(),
                stats.get_max(),
                0,
                (pallete.len() as u64) - 1
            ) as usize;
            let color = pallete.chars().nth(pallete.len() - 1 - color).unwrap(); 
            write!(stdout, "{}", color).unwrap();
        }
    }
}

fn main() {
    let mut vram: Vec<u64> = vec![0; 10_000_000];
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut camera = Camera::new();
    camera.set_scale(4.0);
    // let mut initial_view: bool = true;
    // let mut last_variance: f64 = 0.0;
    let mut stats = Stats::default();
    let mut max_iterations: u64 = 100;
    for c in stdin.keys() {
        let (terminal_width, terminal_height) =
            termion::terminal_size().unwrap();
        let mut screen = Screen {
            terminal_height,
            terminal_width,
            terminal_glyph_ratio: 0.5,
            vram: &mut vram,
        };
        camera.update_aspect_ratio(
            (terminal_width as f64) / (terminal_height as f64)
                * screen.terminal_glyph_ratio,
        );
        // if !initial_view {
        //     max_iterations = if last_variance < stats.get_variance() {
        //         max_iterations - (max_iterations / 10)
        //     } else {
        //         max_iterations + (max_iterations / 10)
        //     };
        //     last_variance = stats.get_variance()
        // }
        // initial_view = false;
        match c.unwrap() {
            Key::Char('q') => camera.adjust_scale(1.0 + 0.1),
            Key::Char('e') => camera.adjust_scale(1.0 - 0.1),
            Key::Char('w') => {
                camera.imag_offset -= camera.imag_span / 10.0 as f64
            }
            Key::Char('s') => {
                camera.imag_offset += camera.imag_span / 10.0 as f64
            }
            Key::Char('a') => {
                camera.real_offset -= camera.real_span / 10.0 as f64
            }
            Key::Char('d') => {
                camera.real_offset += camera.real_span / 10.0 as f64
            }
            Key::Char('r') => {
                camera.reset(1.0, 0.0, 4.0);
                max_iterations = 100;
            }
            Key::Char('1') => {
                max_iterations += ((max_iterations as f64) / 10.0) as u64
            }
            Key::Char('2') => {
                max_iterations -= ((max_iterations as f64) / 10.0) as u64
            }
            Key::Ctrl('z') => break,
            _ => (),
        }
        render_mandelbrot(&mut screen, &camera, &mut stats, max_iterations);
        draw_mandelbrot(&screen, &mut stdout, &stats);

        let mut line: u16 = 1;
        write!(stdout, "{}{:?}", termion::cursor::Goto(1, line), stats)
            .unwrap();
        line += 1;

        write!(
            stdout,
            "{}real_offset: {}",
            termion::cursor::Goto(1, line),
            camera.real_offset
        )
        .unwrap();
        line += 1;

        write!(
            stdout,
            "{}imag_offset: {}",
            termion::cursor::Goto(1, line),
            camera.imag_offset
        )
        .unwrap();
        line += 1;

        write!(
            stdout,
            "{}real_span: {}",
            termion::cursor::Goto(1, line),
            camera.real_span
        )
        .unwrap();
        line += 1;

        write!(
            stdout,
            "{}imag_span: {}",
            termion::cursor::Goto(1, line),
            camera.imag_span
        )
        .unwrap();
        stats = Stats::default();
    }
}

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
    real_span: f64,
    real_step_size: f64,
    imag_span: f64,
    imag_step_size: f64,
    real_offset: f64,
    imag_offset: f64,
    vram: &'a mut Vec<u64>,
}

fn render(screen: &mut Screen, stats: &mut Stats, max_iterations: u64) {
    for imag_step in 0..(screen.terminal_height - 1) as usize {
        let imag = (screen.imag_offset - (screen.imag_span / 2.0))
            + ((imag_step as f64) * screen.imag_step_size);
        for real_step in 0..(screen.terminal_width - 1) as usize {
            let real = (screen.real_offset - (screen.real_span / 2.0))
                + ((real_step as f64) * screen.real_step_size);
            let c = Complex { real, imag };
            let pixel_index =
                (imag_step * screen.terminal_height as usize) + real_step;
            screen.vram[pixel_index] =
                mandelbrot::get_iterations(&c, max_iterations);
            stats.update(screen.vram[pixel_index]);
        }
    }
}

fn draw(screen: &Screen, stdout: &mut RawTerminal<Stdout>, stats: &Stats) {
    write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    // write!(stdout, "{}", termion::clear::All).unwrap();
    let pallete: Vec<char> = vec!['@', '#', '%', '+', '*', '-', '.', '`', ' ']; // TODO optimize
    for imag_step in 0..(screen.terminal_height - 1) as usize {
        for real_step in 0..(screen.terminal_width - 1) as usize {
            let pixel_index =
                (imag_step * screen.terminal_height as usize) + real_step;
            let color = map!(
                screen.vram[pixel_index],
                stats.get_min(),
                stats.get_max(),
                0,
                (pallete.len() as u64) - 1
            ) as usize;
            write!(stdout, "{}", pallete[pallete.len() - 1 - color]).unwrap();
        }
    }
}

fn main() {
    let mut vram: Vec<u64> = vec![0; 10_000_000];
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut real_offset: f64 = 0.0;
    let mut imag_offset: f64 = 0.0;
    let mut view_size: f64 = 4.0;
    // let mut initial_view: bool = true;
    // let mut last_variance: f64 = 0.0;
    let mut stats = Stats::new();
    let mut max_iterations: u64 = 100;
    for c in stdin.keys() {
        // if !initial_view {
        //     max_iterations = if last_variance < stats.get_variance() {
        //         max_iterations - (max_iterations / 10)
        //     } else {
        //         max_iterations + (max_iterations / 10)
        //     };
        //     last_variance = stats.get_variance()
        // }
        // initial_view = false;
        let (terminal_width, terminal_height) =
            termion::terminal_size().unwrap();
        let aspect_ratio = (terminal_width as f64) / (terminal_height as f64);
        let real_span = view_size * aspect_ratio;
        let imag_span = view_size;
        match c.unwrap() {
            Key::Char('q') => view_size += view_size / 10.0,
            Key::Char('e') => view_size -= view_size / 10.0,
            Key::Char('w') => imag_offset -= imag_span / 10.0 as f64,
            Key::Char('s') => imag_offset += imag_span / 10.0 as f64,
            Key::Char('a') => real_offset -= real_span / 10.0 as f64,
            Key::Char('d') => real_offset += real_span / 10.0 as f64,
            Key::Char('1') => {
                max_iterations += ((max_iterations as f64) / 10.0) as u64
            }
            Key::Char('2') => {
                max_iterations -= ((max_iterations as f64) / 10.0) as u64
            }
            Key::Char('r') => (),
            Key::Ctrl('z') => break,
            _ => (),
        }
        let mut screen = Screen {
            terminal_height,
            terminal_width,
            real_span,
            real_step_size: real_span / (terminal_width as f64),
            imag_span,
            imag_step_size: imag_span / (terminal_height as f64),
            real_offset,
            imag_offset,
            vram: &mut vram,
        };
        render(&mut screen, &mut stats, max_iterations);
        draw(&screen, &mut stdout, &stats);

        let mut line: u16 = 1;
        write!(stdout, "{}{:?}", termion::cursor::Goto(1, line), stats)
            .unwrap();
        line += 1;

        write!(
            stdout,
            "{}real_offset: {}",
            termion::cursor::Goto(1, line),
            real_offset
        )
        .unwrap();
        line += 1;

        write!(
            stdout,
            "{}imag_offset: {}",
            termion::cursor::Goto(1, line),
            imag_offset
        )
        .unwrap();
        line += 1;

        write!(
            stdout,
            "{}view_size: {}",
            termion::cursor::Goto(1, line),
            view_size
        )
        .unwrap();
        stats = Stats::new();
    }
}

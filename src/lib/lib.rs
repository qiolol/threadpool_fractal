/*
This file contains code from Programming Rust by Jim Blandy and Jason Orendorff
(O'Reilly), copyright 2018 Jim Blandy and Jason Orendorff, 978-1-491-92728-1.
*/

use std::io::Write;
use std::sync::{Arc, Mutex};

use num_complex::Complex;

mod threadpool;
mod mandelbrot;
mod colors;

pub struct Args {
    pub limit: u32,
    pub image_width: usize,
    pub image_height: usize,
    pub complex_upper_left_corner: Complex<f64>,
    pub complex_lower_right_corner: Complex<f64>,
    pub output_filename: String,
}

fn print_usage(exe: &str) {
    writeln!(std::io::stderr(),
        "Usage: mandelbrot <output_filename> <resolution> <upper_left_c> <lower_right_c> <limit>\n"
    ).unwrap();
    writeln!(std::io::stderr(),
        "\t- output_filename is the filename of output image\
        \n\t- resolution defines the dimensions of output image, in pixels\
        \n\t- upper_left_c is upper left corner of the complex plane to render\
        \n\t- lower_right_c is lower right corner of the complex plane to render\
        \n\t- limit is the number of iterations with which to test points (higher\
        is slower but more accurate)\n"
    ).unwrap();
    writeln!(std::io::stderr(),
        "Example: {} frac.png 1000x1000 -0.245178,-0.650185 -0.244486,-0.649417 250",
        exe
    ).unwrap();
}

/// Validates and returns input in an `Args` struct
pub fn parse_input() -> Args {
    let got_args: Vec<String> = std::env::args().collect();

    if got_args.len() == 6 {
        let output_filename: &str = &got_args[1];
        let resolution: (usize, usize) = parse_pair(&got_args[2], 'x')
            .expect("error parsing image resolution");
        let complex_upper_left_corner: Complex<f64> = parse_complex(&got_args[3])
            .expect("error parsing upper left complex bound");
        let complex_lower_right_corner: Complex<f64> = parse_complex(&got_args[4])
            .expect("error parsing lower right complex bound");
        let limit: u32 = got_args[5].parse().unwrap();

        let ret_args = Args {
            limit: limit,
            image_width: resolution.0,
            image_height: resolution.1,
            complex_upper_left_corner: complex_upper_left_corner,
            complex_lower_right_corner: complex_lower_right_corner,
            output_filename: output_filename.to_string()
        };

        return ret_args;
    }

    print_usage(&got_args[0]);

    std::process::exit(1);
}

/// Parses the string `s` to read a coordinate pair, like `"400x600"` or `"1.0,0.5"`,
/// and returns the pair as `Some<(x, y)>` or `None` if parsing failed
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the char argument `separator` and <left> and <right> are both strings that
/// can be parsed by `T::from_str`.
pub fn parse_pair<T: std::str::FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => { // index is separator's position
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("20,10", ','), Some((20, 10)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Parses a pair of floats separated by a comma as a complex number
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Renders a rectangle of the Mandelbrot set with a single thread
///
/// Every pixel in `pixels` is treated as a point on the complex plane, is
/// tested for membership in the set, and colored accordingly per the given
/// color theme.
///
/// `pixels` is the output buffer, containing a rectangle of pixels
/// `complex_upper_left_bound` and `complex_lower_right_bound` designate the
/// area on the complex plane covered by the rectangle
/// `limit` is the maximum number of iterations used to test each pixel
/// (the higher it is, the more accurate the test)
pub fn render_singlethreaded(limit: u32,
                             complex_upper_left_corner: Complex<f64>,
                             complex_lower_right_corner: Complex<f64>,
                             pixels: Arc<Mutex<image::RgbImage>>) {
    let mut iterations: u32;
    let color_theme = crate::colors::grayscale_theme();
    let flux = 1; // magic
    let width = (*pixels).lock().unwrap().width() as usize;
    let height = (*pixels).lock().unwrap().height() as usize;

    for (x, y, pixel) in (*pixels.lock().unwrap()).enumerate_pixels_mut() {
        let complex_point = crate::mandelbrot::pixel_to_complex_point((x as usize, y as usize),
                                                                      width, height,
                                                                      complex_upper_left_corner,
                                                                      complex_lower_right_corner);
        iterations = crate::mandelbrot::escape_time(complex_point, limit);

        *pixel = crate::colors::iterations_to_color(iterations, limit, &color_theme, flux);
    }
}

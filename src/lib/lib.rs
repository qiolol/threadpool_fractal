/*
This file contains code from Programming Rust by Jim Blandy and Jason Orendorff
(O'Reilly), copyright 2018 Jim Blandy and Jason Orendorff, 978-1-491-92728-1.
*/

use num_complex::Complex;
use image::RgbImage;

mod threadpool;
mod mandelbrot;
mod colors;

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

/// Writes a buffer of pixels, `pixels`, to `filename`
pub fn write_image(filename: &str,
                   pixels: &RgbImage) {
    pixels.save(filename).expect("error writing to image file");
}

pub fn singlethreaded(filename: &str,
                      resolution: (usize, usize),
                      complex_upper_left_bound: Complex<f64>,
                      complex_lower_right_bound: Complex<f64>,
                      limit: u32) {
    let (width, height) = (resolution.0, resolution.1);
    let mut pixels = RgbImage::new(width as u32, height as u32); // pixel buffer

    // fill pixel buffer
    mandelbrot::render(&mut pixels,
                       complex_upper_left_bound,
                       complex_lower_right_bound,
                       limit);

    // write points to image
    write_image(&filename, &pixels);
}

/*
This file contains code from Programming Rust by Jim Blandy and Jason Orendorff
(O'Reilly), copyright 2018 Jim Blandy and Jason Orendorff, 978-1-491-92728-1.
*/

use std::io::Write;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use num_complex::Complex;
use image::{Rgb, RgbImage};

mod threadpool;
mod mandelbrot;
pub mod colors;

/// Parsed/validated arguments
pub struct Args {
    pub limit: u32,
    pub threads: u32,
    pub image_width: usize,
    pub image_height: usize,
    pub complex_upper_left_corner: Complex<f64>,
    pub complex_lower_right_corner: Complex<f64>,
    pub output_filename: String,
    pub color_theme: Vec<Rgb<u8>>,
}

fn print_usage(exe: &str, color_themes: HashMap<&str, Vec<Rgb<u8>>>) {
    writeln!(std::io::stderr(),
        "Usage: mandelbrot <output_filename> <resolution> <upper_left_c> \
        <lower_right_c> <limit> <threads> <color_theme>\n"
    ).unwrap();
    writeln!(std::io::stderr(),
        "\t- output_filename is the filename of output image\
        \n\t- resolution defines the dimensions of output image, in pixels\
        \n\t- upper_left_c is upper left corner of the complex plane to render\
        \n\t- lower_right_c is lower right corner of the complex plane to render\
        \n\t- limit is the number of iterations with which to test points (higher \
        is slower but more accurate)\
        \n\t- threads is the number of threads to use\
        \n\t- color_theme is one of:"
    ).unwrap();
    // List available color themes
    for theme_name in color_themes.keys() {
        writeln!(std::io::stderr(), "\t\t- {}", theme_name).unwrap();
    }
    writeln!(std::io::stderr(),
        "\n\tExample:\n\t{} frac.png 2000x2000 -0.245178,-0.650185 -0.244486,-0.649417 \
        350 6 raspberry_acid",
        exe
    ).unwrap();
}

/// Validates and returns input in an `Args` struct
pub fn parse_input() -> Args {
    let got_args: Vec<String> = std::env::args().collect();
    let color_themes = HashMap::from([
        ("grayscale",       crate::colors::grayscale()),
        ("space",           crate::colors::space()),
        ("fire",            crate::colors::fire()),
        ("k8_peacock",      crate::colors::k8_peacock()),
        ("usa",             crate::colors::usa()),
        ("raspberry_acid",  crate::colors::raspberry_acid()),
        ("mojave",          crate::colors::mojave()),
        ("houndeye",        crate::colors::houndeye()),
    ]);

    if got_args.len() == 8 {
        let output_filename: &str = &got_args[1];
        let resolution: (usize, usize) = parse_pair(&got_args[2], 'x')
            .expect("error parsing image resolution");
        let complex_upper_left_corner: Complex<f64> = parse_complex(&got_args[3])
            .expect("error parsing upper left complex bound");
        let complex_lower_right_corner: Complex<f64> = parse_complex(&got_args[4])
            .expect("error parsing lower right complex bound");
        let limit: u32 = got_args[5].parse().unwrap();
        let threads: u32 = got_args[6].parse().unwrap();
        let color_theme: &str = &got_args[7];

        if color_themes.contains_key(color_theme) {
            let ret_args = Args {
                limit: limit,
                threads: threads,
                image_width: resolution.0,
                image_height: resolution.1,
                complex_upper_left_corner: complex_upper_left_corner,
                complex_lower_right_corner: complex_lower_right_corner,
                output_filename: output_filename.to_string(),
                color_theme: color_themes.get(color_theme).unwrap().to_vec()
            };
    
            return ret_args;
        }
    }

    print_usage(&got_args[0], color_themes);

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
/// `color_theme`.
///
/// `limit` is the maximum number of iterations used to test each pixel
/// (the higher it is, the more accurate the test)
/// `complex_upper_left_bound` and `complex_lower_right_bound` designate the
/// area on the complex plane covered by the rectangle
/// `pixels` is the output buffer, containing a rectangle of pixels
// `color_theme` is the palette we'll use to color pixels
pub fn render_singlethreaded(
    limit: u32,
    complex_upper_left_corner: Complex<f64>,
    complex_lower_right_corner: Complex<f64>,
    pixels: Arc<Mutex<RgbImage>>,
    color_theme: Vec<Rgb<u8>>
) {
    let width = pixels.lock().unwrap().width();
    let height = pixels.lock().unwrap().height();

    for (x, y, pixel) in pixels.lock().unwrap().enumerate_pixels_mut() {
        let complex_point = crate::mandelbrot::pixel_to_complex_point(
            (x, y),
            width, height,
            complex_upper_left_corner,
            complex_lower_right_corner
        );
        let iterations = crate::mandelbrot::escape_time(complex_point, limit);

        *pixel = crate::colors::iterations_to_color(
            iterations,
            limit,
            &color_theme
        );
    }
}

/// Container for a pixel and its coordinates in the output image
struct PixelData {
    pub pixel: Rgb<u8>,
    pub x: u32,
    pub y: u32,
}

/// Splits `pixels` into segments such that there's one segment per thread
fn divide_image_into_segments(
    pixels: &mut RgbImage,
    width: u32,
    height: u32,
    threads: u32
) -> Vec<Vec<PixelData>> {
    let mut segments: Vec<Vec<PixelData>> = Vec::with_capacity(threads as usize);
    let total_pixels = width * height;
    let pixels_per_segment = (total_pixels / threads + 1) as usize;

    for _ in 0..threads {
        segments.push(Vec::with_capacity(pixels_per_segment as usize));
    }

    // Assign each segment its pixels
    let mut pixels_assigned = 0;
    let mut curr_segment = 0;

    for (x, y, pixel) in pixels.enumerate_pixels_mut() {
        segments[curr_segment].push(PixelData {
            pixel: *pixel,
            x: x,
            y: y
        });

        pixels_assigned += 1;

        if pixels_assigned == pixels_per_segment {
            // Begin filling next segment
            curr_segment += 1;
            pixels_assigned = 0;
        }
    }

    return segments;
}

#[test]
fn test_divide_image_into_segments() {
    let mut img = RgbImage::new(3, 3);
    let width = img.width();
    let height = img.height();
    let threads = 5;

    let segments = divide_image_into_segments(
        &mut img,
        width, height,
        threads
    );

    // Correct number of segments
    assert_eq!(segments.len(), 5);

    // Correct length of segments
    assert_eq!(segments[0].len(), 2);
    assert_eq!(segments[1].len(), 2);
    assert_eq!(segments[2].len(), 2);
    assert_eq!(segments[3].len(), 2);
    assert_eq!(segments[4].len(), 1);

    // Correct pixel coordinates in first segment
    assert_eq!(segments[0][0].x, 0);
    assert_eq!(segments[0][0].y, 0);

    assert_eq!(segments[0][1].x, 1);
    assert_eq!(segments[0][1].y, 0);

    // Correct pixel coordinates in second segment
    assert_eq!(segments[1][0].x, 2);
    assert_eq!(segments[1][0].y, 0);

    assert_eq!(segments[1][1].x, 0);
    assert_eq!(segments[1][1].y, 1);

    // Correct pixel coordinates in third segment
    assert_eq!(segments[2][0].x, 1);
    assert_eq!(segments[2][0].y, 1);

    assert_eq!(segments[2][1].x, 2);
    assert_eq!(segments[2][1].y, 1);

    // Correct pixel coordinates in fourth segment
    assert_eq!(segments[3][0].x, 0);
    assert_eq!(segments[3][0].y, 2);

    assert_eq!(segments[3][1].x, 1);
    assert_eq!(segments[3][1].y, 2);

    // Correct pixel coordinates in fifth segment
    assert_eq!(segments[4][0].x, 2);
    assert_eq!(segments[4][0].y, 2);
}

/// Renders a rectangle of the Mandelbrot set with `threads` threads by
/// breaking up the pixels into `threads` segments so that each thread will
/// have one segment to process
pub fn render_multithreaded_preallocated_segments(
    limit: u32,
    complex_upper_left_corner: Complex<f64>,
    complex_lower_right_corner: Complex<f64>,
    pixels: Arc<Mutex<RgbImage>>,
    threads: u32,
    color_theme: Vec<Rgb<u8>>
) {
    let width = pixels.lock().unwrap().width();
    let height = pixels.lock().unwrap().height();

    // Divide image into segments
    let segments: Vec<Vec<PixelData>> = divide_image_into_segments(
        &mut *pixels.lock().unwrap(),
        width, height,
        threads
    );

    // Let threads process segments
    let mut thread_handles = vec![];

    for mut segment in segments {
        let loop_pixels = Arc::clone(&pixels);
        let loop_theme = color_theme.clone();
        
        thread_handles.push(
            std::thread::spawn(move || {
                // Process segment
                for mut pixel_data in &mut segment {
                    let complex_point = crate::mandelbrot::pixel_to_complex_point(
                        (pixel_data.x, pixel_data.y),
                        width, height,
                        complex_upper_left_corner,
                        complex_lower_right_corner
                    );
                    let iterations = crate::mandelbrot::escape_time(complex_point, limit);

                    pixel_data.pixel = crate::colors::iterations_to_color(
                        iterations,
                        limit,
                        &loop_theme
                    );
                }

                // Write processed segment to image
                for pixel_data in segment {
                    *loop_pixels.lock().unwrap()
                        .get_pixel_mut(pixel_data.x, pixel_data.y) = pixel_data.pixel;
                }
            })
        );
    }

    // Join all threads (wait for them to finish)
    for handle in thread_handles {
        handle.join().unwrap();
    }
}

/// Splits `pixels` into rows
fn divide_image_into_rows(
    pixels: &mut RgbImage,
    width: u32,
    height: u32
) -> Vec<Vec<PixelData>> {
    let mut rows: Vec<Vec<PixelData>> = Vec::with_capacity(height as usize);

    for _ in 0..height {
        rows.push(Vec::with_capacity(width as usize));
    }

    // Assign each row its pixels
    for (_, row) in pixels.enumerate_rows_mut() {
        for (x, y, pixel) in row {
            rows[y as usize].push(PixelData {
                pixel: *pixel,
                x: x,
                y: y
            });
        }
    }

    return rows;
}

#[test]
fn test_divide_image_into_rows() {
    let mut img = RgbImage::new(3, 2);
    let width = img.width();
    let height = img.height();

    let rows = divide_image_into_rows(
        &mut img,
        width, height
    );

    // Correct number of rows
    assert_eq!(rows.len(), 2);

    // Correct length of rows
    assert_eq!(rows[0].len(), 3);
    assert_eq!(rows[1].len(), 3);

    // Correct pixel coordinates in first row
    assert_eq!(rows[0][0].x, 0);
    assert_eq!(rows[0][0].y, 0);

    assert_eq!(rows[0][1].x, 1);
    assert_eq!(rows[0][1].y, 0);

    assert_eq!(rows[0][2].x, 2);
    assert_eq!(rows[0][2].y, 0);

    // Correct pixel coordinates in second row
    assert_eq!(rows[1][0].x, 0);
    assert_eq!(rows[1][0].y, 1);

    assert_eq!(rows[1][1].x, 1);
    assert_eq!(rows[1][1].y, 1);

    assert_eq!(rows[1][2].x, 2);
    assert_eq!(rows[1][2].y, 1);
}

/// Renders a rectangle of the Mandelbrot set with `threads` threads by
/// breaking up the pixels into rows and tossing the rows into a thread pool
/// for processing
pub fn render_multithreaded_pooled_rows(
    limit: u32,
    complex_upper_left_corner: Complex<f64>,
    complex_lower_right_corner: Complex<f64>,
    pixels: Arc<Mutex<RgbImage>>,
    threads: u32,
    color_theme: Vec<Rgb<u8>>
) {
    let width = pixels.lock().unwrap().width();
    let height = pixels.lock().unwrap().height();

    // Divide image into rows
    let rows: Vec<Vec<PixelData>> = divide_image_into_rows(
        &mut *pixels.lock().unwrap(),
        width, height
    );

    // Let threads process rows
    let pool = crate::threadpool::ThreadPool::new(threads as usize);

    for mut row in rows {
        let loop_pixels = Arc::clone(&pixels);
        let loop_theme = color_theme.clone();

        pool.execute(move || {
            // Process row
            for mut pixel_data in &mut row {
                let complex_point = crate::mandelbrot::pixel_to_complex_point(
                    (pixel_data.x, pixel_data.y),
                    width, height,
                    complex_upper_left_corner,
                    complex_lower_right_corner
                );
                let iterations = crate::mandelbrot::escape_time(complex_point, limit);

                pixel_data.pixel = crate::colors::iterations_to_color(
                    iterations,
                    limit,
                    &loop_theme
                );
            }

            // Write processed row to image
            for pixel_data in row {
                *loop_pixels.lock().unwrap()
                    .get_pixel_mut(pixel_data.x, pixel_data.y) = pixel_data.pixel;
            }
        });
    }
}

/// Renders a rectangle of the Mandelbrot set with `threads` threads by
/// tossing all the pixels into a thread pool for processing
pub fn render_multithreaded_pooled_pixels(
    limit: u32,
    complex_upper_left_corner: Complex<f64>,
    complex_lower_right_corner: Complex<f64>,
    pixels: Arc<Mutex<RgbImage>>,
    threads: u32,
    color_theme: Vec<Rgb<u8>>
) {
    let width = pixels.lock().unwrap().width();
    let height = pixels.lock().unwrap().height();

    // Let threads process pixels
    let pool = crate::threadpool::ThreadPool::new(threads as usize);

    for (x, y, _) in pixels.lock().unwrap().enumerate_pixels_mut() {
        let loop_pixels = Arc::clone(&pixels);
        let loop_theme = color_theme.clone();

        pool.execute(move || {
            // Process pixel
            let complex_point = crate::mandelbrot::pixel_to_complex_point(
                (x, y),
                width, height,
                complex_upper_left_corner,
                complex_lower_right_corner
            );
            let iterations = crate::mandelbrot::escape_time(complex_point, limit);

            // Write processed pixel to image
            *loop_pixels.lock().unwrap()
                .get_pixel_mut(x, y) = crate::colors::iterations_to_color(
                    iterations,
                    limit,
                    &loop_theme
                );
        });
    }
}

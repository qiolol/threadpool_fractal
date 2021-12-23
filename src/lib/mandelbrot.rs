/*
This file contains code from Programming Rust by Jim Blandy and Jason Orendorff
(O'Reilly), copyright 2018 Jim Blandy and Jason Orendorff, 978-1-491-92728-1.
*/

use num_complex::Complex;

/// Returns the number of iterations, up to the given `limti`, it took for `c`
/// to escape the Mandelbrot set
///
/// If the return value is `limit`, `c` did not escape within `limit` iterations,
/// indicating that `c` is probably in the set.
pub fn escape_time(c: Complex<f64>, limit: u32) -> u32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut i: u32 = 0;

    while i < limit {
        z = z * z + c;

        // A classic shortcut this code uses is the wisdom that, if `z` ever
        // leaves a circle of radius 2 centered on the origin, it will fly out
        // to infinity eventually, and thus prove itself to be outside the set.
        if z.norm_sqr() > 4.0 {
            return i;
        }

        i += 1;
    }

    // Down here, i == limit
    return limit;
}

/// Returns the point on the complex plane corresponding to the given image
/// pixel coordinates
///
/// This converts "image space" to "complex number space".
///
/// `pixel_coords` is an (x, y) pair representing a pixel in the output image
/// `width` and `height` are the dimensions of the image in pixels
/// `complex_upper_left_bound` and `complex_lower_right_bound` designate the
/// area on the complex plane covered by the output image
pub fn pixel_to_complex_point(pixel_coords: (usize, usize),
                  width: usize,
                  height: usize,
                  complex_upper_left_bound: Complex<f64>,
                  complex_lower_right_bound: Complex<f64>)
    -> Complex<f64> {
    let (pixel_x, pixel_y) = (pixel_coords.0, pixel_coords.1);
    let real_scale = complex_lower_right_bound.re - complex_upper_left_bound.re;
    let imag_scale = complex_upper_left_bound.im - complex_lower_right_bound.im;

    // I have no idea how/why this calculation works, but the book helpfully
    // mentions that, while pixel_y increases as we go down (in keeping with
    // graphics code tradition), the imaginary component DECREASES, and vice
    // versa...
    return Complex {
        re: complex_upper_left_bound.re + (pixel_x as f64 * real_scale as f64 / width as f64),
        im: complex_upper_left_bound.im - (pixel_y as f64 * imag_scale as f64 / height as f64)
    }
}

#[test]
fn test_pixel_to_complex_point() {
    assert_eq!(
        pixel_to_complex_point(
            (25, 75),
            100, 100,
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

/*
This file contains code from Programming Rust by Jim Blandy and Jason Orendorff
(O'Reilly), copyright 2018 Jim Blandy and Jason Orendorff, 978-1-491-92728-1.
*/

use num_complex::Complex;

/// Returns `Some(i)` if the `c` escaped (wasn't in) the Mandelbrot set or `None`
/// if `c` never escaped (seems to be in the set) in `limit` iterations
///
/// `i` is the number of iterations it took `c` to escape.
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;
        // A classic shortcut this code uses is the wisdom that, if `z` ever
        // leaves a circle of radius 2 centered on the origin, it will fly out
        // to infinity eventually, and thus prove itself to be outside the set.
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    return None;
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
fn pixel_to_point(pixel_coords: (usize, usize),
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
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (25, 75),
            100, 100,
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

/// Renders a rectangle of the Mandelbrot set
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
pub fn render(pixels: &mut image::RgbImage,
              complex_upper_left_bound: Complex<f64>,
              complex_lower_right_bound: Complex<f64>,
              limit: u32) {
    let mut esc: u32;
    let color_theme = crate::colors::grayscale_theme();
    let flux = 1; // magic
    let (width, height) = (pixels.width() as usize, pixels.height() as usize);

    for y in 0 .. height {
        for x in 0 .. width {
            let complex_point = pixel_to_point((x, y),
                                               width, height,
                                               complex_upper_left_bound,
                                               complex_lower_right_bound);

            match escape_time(complex_point, limit) {
                None => esc = 0,
                Some(count) => esc = limit - count
            };

            pixels[(x as u32, y as u32)] = crate::colors::color_map(esc,
                                                                    &color_theme,
                                                                    flux);
        }
    }
}

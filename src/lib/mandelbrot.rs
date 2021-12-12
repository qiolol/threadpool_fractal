use image;
use num_complex;
use std::sync::{Arc, Mutex};

/// Computes a pixel of the Mandelbrot set
pub fn compute_pixel(imgbuf: Arc<Mutex<image::RgbImage>>, x: u32, y: u32, scale_x: f32, scale_y: f32) {
    let c_x = x as f32 * scale_x - 1.5;               // oh, certainly, oh, yes yes
    let c_y = y as f32 * scale_y - 1.5;               // oooh, yes yes, yeeeees, n-no--*CERTAINLY*.
                                                      // YEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEES.
    let c = num_complex::Complex::new(c_x, c_y);      // compute the centered complex coordinates
    let mut z = num_complex::Complex::new(c_x, c_y);  // **INDEED**.
                                                      // https://youtu.be/8giyln7F_Uk?t=106
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
        z = z * z + c;
        i += 1;
    }

    let mut imgbuf_mutex_guard = (*imgbuf).lock().unwrap();
    let pixel = (*imgbuf_mutex_guard).get_pixel_mut(x, y);
    let image::Rgb(data) = *pixel;
    *pixel = image::Rgb([data[0], i as u8, data[2]]);
}

use std::sync::{Arc, Mutex};

use image;
use num_complex;

/// Compute a pixel of the Mandelbrot set
fn compute_pixel(imgbuf: Arc<Mutex<image::RgbImage>>, x: u32, y: u32, scale_x: f32, scale_y: f32) {
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

/// Compute result serially (single-threaded)
#[allow(dead_code)]
fn serial(imgbuf: Arc<Mutex<image::RgbImage>>, img_x: u32, img_y: u32, scale_x: f32, scale_y: f32) {
    for x in 0..img_x {
        for y in 0..img_y {
            compute_pixel(Arc::clone(&imgbuf), x, y, scale_x, scale_y);
        }
    }
}

/// Compute result with parallel threads
fn parallel(imgbuf: Arc<Mutex<image::RgbImage>>, img_x: u32, img_y: u32, scale_x: f32, scale_y: f32) {
    let pool = threadpool_fractal::ThreadPool::new(4);

    // A redundant loop to demonstrate reading image data
    for x in 0..img_x {
        for y in 0..img_y {
            let imgbuf_inner_arc = Arc::clone(&imgbuf);

            pool.execute(move || {
                compute_pixel(imgbuf_inner_arc, x, y, scale_x, scale_y);
            });
        }
    }
}

fn main() {
    // image dimensions
    let img_x = 800;
    let img_y = 800;
    // dimensions of the view on the complex plane
    let complex_plane_x = 3.0;
    let complex_plane_y = 3.0;
    // scale_n = (complex plane displacement / image pixel displacement), along the n axis
    let scale_x = complex_plane_x / img_x as f32;
    let scale_y = complex_plane_y / img_y as f32;

    // create image (wrapped in a Mutex and Arc for multithread readiness)
    let imgbuf = Arc::new(Mutex::new(image::ImageBuffer::new(img_x, img_y)));

    // color the canvas as a red-blue gradient
    for (x, y, pixel) in (*imgbuf.lock().unwrap()).enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }
    
    // serial(Arc::clone(&imgbuf), img_x, img_y, scale_x, scale_y); // single-threaded
    parallel(Arc::clone(&imgbuf), img_x, img_y, scale_x, scale_y); // multithreaded

    // write image to file
    (*imgbuf.lock().unwrap()).save("fractal.png").unwrap();
}

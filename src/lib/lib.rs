use std::sync::{Arc, Mutex};

mod threadpool;
mod mandelbrot;

/// Computes an image of the Mandelbrot set serially
#[allow(dead_code)]
pub fn serial(imgbuf: Arc<Mutex<image::RgbImage>>, img_x: u32, img_y: u32, scale_x: f32, scale_y: f32) {
    for x in 0..img_x {
        for y in 0..img_y {
            mandelbrot::compute_pixel(Arc::clone(&imgbuf), x, y, scale_x, scale_y);
        }
    }
}

/// Computes an image of the Mandelbrot set with parallel threads
pub fn parallel(imgbuf: Arc<Mutex<image::RgbImage>>, img_x: u32, img_y: u32, scale_x: f32, scale_y: f32) {
    let pool = threadpool::ThreadPool::new(4);

    for x in 0..img_x {
        for y in 0..img_y {
            // pass pixel computation to the thread pool
            let imgbuf_inner_arc = Arc::clone(&imgbuf);

            pool.execute(move || {
                mandelbrot::compute_pixel(imgbuf_inner_arc, x, y, scale_x, scale_y);
            });
        }
    }
}

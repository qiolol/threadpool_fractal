use std::sync::{Arc, Mutex};

//use threadpool_fractal::serial; // unused
use threadpool_fractal::parallel;

fn main() {
    // image dimensions
    let img_x = 800;
    let img_y = 800;
    // dimensions of the view on the complex plane
    let complex_plane_x = 3.0;
    let complex_plane_y = 3.0;
    // scale_n = (complex plane displacement / image pixel displacement),
    // along the x and y axis
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

    //serial(Arc::clone(&imgbuf), img_x, img_y, scale_x, scale_y); // single-threaded
    parallel(Arc::clone(&imgbuf), img_x, img_y, scale_x, scale_y); // multithreaded

    // write image to file
    (*imgbuf.lock().unwrap()).save("fractal.png").unwrap();
}

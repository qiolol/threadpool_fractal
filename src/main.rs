/*
This code concurrently renders an image of an approximation of the Mandelbrot set.
(Actually, this is a lie; it used to do it concurrently, but, for now, it's only
serial again. : D) An image of the set is made by treating each pixel of the image as
a point on the complex plane and seeing whether that point is in the set.

The Mandelbrot set is the set of complex numbers `c` for which `z` does not
fly out to infinity (and instead circles around the origin) when calculating
`z = z * z + c` in an infinite loop. Less-than-infinite iterations yield
less-than-exact approximations of the set, with more iterations yielding more
accurate approximations.
*/

use std::sync::{Arc, Mutex};

use image::RgbImage;

fn main() {
    let args = threadpool_fractal::parse_input();

    let output_image = Arc::new( // Gives shared ownership of Mutex
        Mutex::new( // Thread-safes the RgbImage
            RgbImage::new(args.image_width as u32, args.image_height as u32)
        )
    );

    threadpool_fractal::render_singlethreaded(args.limit,
                                              args.complex_upper_left_corner,
                                              args.complex_lower_right_corner,
                                              Arc::clone(&output_image));

    // Write image to file
    (*output_image.lock().unwrap()).save(args.output_filename).expect("error writing to image file");
}

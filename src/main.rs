use std::sync::{Arc, Mutex};

fn main() {
    let args = threadpool_fractal::parse_input();
    let output_image = Arc::new( // Gives shared ownership of Mutex
        Mutex::new( // Thread-safes mutability of image
            image::RgbImage::new(args.image_width as u32, args.image_height as u32)
        )
    );
    let color_theme = threadpool_fractal::colors::grayscale_theme();
    let number_of_threads = 6;

    // threadpool_fractal::render_singlethreaded(
    //     args.limit,
    //     args.complex_upper_left_corner,
    //     args.complex_lower_right_corner,
    //     Arc::clone(&output_image),
    //     color_theme
    // );
    
    threadpool_fractal::render_multithreaded_preallocated_segments(
        args.limit,
        args.complex_upper_left_corner,
        args.complex_lower_right_corner,
        Arc::clone(&output_image),
        number_of_threads,
        color_theme
    );

    // Write image to file
    output_image.lock().unwrap().save(args.output_filename)
        .expect("error writing to image file");
}

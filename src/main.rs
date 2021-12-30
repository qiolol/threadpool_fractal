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

    // 🐢 Slowest
    // This is just one thread, doing all the work, one pixel at a time.
    //threadpool_fractal::render_singlethreaded(
    //    args.limit,
    //    args.complex_upper_left_corner,
    //    args.complex_lower_right_corner,
    //    Arc::clone(&output_image),
    //    color_theme
    //);
    
    // 🐇 Faster
    // This partitions the image into segments of pixels and assigns one segment
    // per thread, preallocating the workload in a broad way.
    //
    // This is faster since threads (on a multi-core CPU) are doing the work in
    // parallel.
    //threadpool_fractal::render_multithreaded_preallocated_segments(
    //    args.limit,
    //    args.complex_upper_left_corner,
    //    args.complex_lower_right_corner,
    //    Arc::clone(&output_image),
    //    number_of_threads,
    //    color_theme
    //);

    // 🐇💢 Slightly faster
    // This partitions the image into rows of pixels and tosses all the rows
    // into the thread pool for threads to snatch and process.
    //
    // This is even faster (slightly) since some areas of the image get rendered
    // faster than others. Pixels that correspond to complex points which escape
    // the set right away are rendered very quickly. Rows replete with such
    // pixels get processed very quickly. With row-by-row granularity, the total
    // processing time is more evenly-distributed among the threads than when
    // threads get large segments of pixels. Some segments may be full of such
    // pixels, and some not, so threads with "fast" segments that finish early
    // have nothing left to do while threads with "slow" segments keep crunching
    // away.
    threadpool_fractal::render_multithreaded_pooled_rows(
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

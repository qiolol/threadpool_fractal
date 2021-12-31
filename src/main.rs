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
    /*
    threadpool_fractal::render_singlethreaded(
       args.limit,
       args.complex_upper_left_corner,
       args.complex_lower_right_corner,
       Arc::clone(&output_image),
       color_theme
    );
    */
    
    // 🐇 Faster
    // This partitions the image into segments of pixels and assigns one segment
    // per thread, preallocating the workload in a broad way.
    //
    // This is faster since threads (on a multi-core CPU) are doing the work in
    // parallel.
    /*
    threadpool_fractal::render_multithreaded_preallocated_segments(
       args.limit,
       args.complex_upper_left_corner,
       args.complex_lower_right_corner,
       Arc::clone(&output_image),
       number_of_threads,
       color_theme
    );
    */

    // 🐇++ Slightly faster
    // This partitions the image into rows of pixels and tosses all the rows
    // into the thread pool for threads to snatch up, process, and snatch up
    // more when they finish, until no more rows remain in the pool.
    //
    // This is even faster (slightly) since:
    //
    //     1. Some pixels get rendered faster than others
    //
    //     2. With row-by-row granularity, the workload is more evenly
    //        distributed among the threads
    //
    // The reason for 1 is that pixels that correspond to complex points which
    // escape the set right away are rendered very quickly, since they're done
    // once they escape. Call these "fast pixels".
    //
    // The reason for 2 is that rows replete with fast pixels are rendered very
    // quickly. Before, when threads were assigned large segments of pixels,
    // a thread that got a "fast" segment made up of fast rows would finish and
    // then be idle while threads that got "slow" segments kept crunching. With
    // row-by-row granularity, such idle threads instead pick up another row to
    // process.
    threadpool_fractal::render_multithreaded_pooled_rows(
        args.limit,
        args.complex_upper_left_corner,
        args.complex_lower_right_corner,
        Arc::clone(&output_image),
        number_of_threads,
        color_theme
    );

    // 🐇-- Less fast
    // This tosses all the individual pixels into the thread pool.
    //
    // Since finer (row) granularity was faster than broader (large image
    // segment) granularity, you'd think that *even finer* (pixel) granularity
    // would be faster still! But, actually, this is slower than row-based and
    // segment-based multithreading.
    //
    // I think this is because of all the access to the shared image. After a
    // thread processes a pixel (which takes very little time), it requests
    // access to the image to write the result, blocking the other threads. All
    // threads likely spend most of their time waiting for other threads to give
    // up their lock on the shared image than on actual work, resulting in a
    // slow-down. I assumed this slow-down would be so severe that this
    // approaches singlethreaded performance, but it's still about twice as fast
    // as singlethreaded.
    /*
    threadpool_fractal::render_multithreaded_pooled_pixels(
        args.limit,
        args.complex_upper_left_corner,
        args.complex_lower_right_corner,
        Arc::clone(&output_image),
        number_of_threads,
        color_theme
    );
    */

    // Write image to file
    output_image.lock().unwrap().save(args.output_filename)
        .expect("error writing to image file");
}

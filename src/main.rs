/*
This file contains code from Programming Rust by Jim Blandy and Jason Orendorff
(O'Reilly), copyright 2018 Jim Blandy and Jason Orendorff, 978-1-491-92728-1.

This code concurrently renders an image of an approximation of the Mandelbrot set.
An image of the set is made by treating each pixel of the image as a point on
the complex plane and seeing whether that point is in the set.

The Mandelbrot set is the set of complex numbers `c` for which `z` does not
fly out to infinity (and instead circles around the origin) when calculating
`z = z * z + c` in an infinite loop. Less-than-infinite iterations yield
less-than-exact approximations of the set, with more iterations yielding more
accurate approximations.
*/

use std::io::Write;

use num_complex::Complex;

fn check_input() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 6 {
        writeln!(std::io::stderr(),
                 "Usage: mandelbrot <output_image> <resolution> <upper_left_c> <lower_right_c> <limit>\n"
        ).unwrap();
        writeln!(std::io::stderr(),
                 "upper_left_c and lower_right_c are complex numbers bounding the area of interest on the complex plane.\n"
        ).unwrap();
        writeln!(std::io::stderr(),
                 "Example: {} frac.png 1000x1000 -0.245178,-0.650185 -0.244486,-0.649417 250",
                 args[0]
        ).unwrap();

        std::process::exit(1);
    }

    return args;
}

fn main() {
    let args = check_input();

    let filename: &str = &args[1];
    let resolution: (usize, usize) = threadpool_fractal::parse_pair(&args[2], 'x')
        .expect("error parsing image resolution");
    let upper_left_c: Complex<f64> = threadpool_fractal::parse_complex(&args[3])
        .expect("error parsing upper left complex bound");
    let lower_right_c: Complex<f64> = threadpool_fractal::parse_complex(&args[4])
        .expect("error parsing lower right complex bound");
    let limit: u32 = args[5].parse().unwrap();

    threadpool_fractal::singlethreaded(
        filename,
        resolution,
        upper_left_c,
        lower_right_c,
        limit
    );
}

/*
This file takes heavy inspiration from, and uses techniques credited to, the
color mapping code at: https:github.com/Kate-Painter/BunnyFrac/blob/main/src/color.rs
*/

#![allow(dead_code)]

use image::Rgb;

// Colors
const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const DARK_GRAY: Rgb<u8> = Rgb([76, 76, 76]);
const GRAY: Rgb<u8> = Rgb([127, 127, 127]);
const LIGHT_GRAY: Rgb<u8> = Rgb([178, 178, 178]);

const RED: Rgb<u8> = Rgb([255, 0, 0]);
const ORANGE: Rgb<u8> = Rgb([255, 127, 0]);
const YELLOW: Rgb<u8> = Rgb([255, 255, 0]);

const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
const VIOLET: Rgb<u8> = Rgb([127, 0, 255]);

const PURPLE: Rgb<u8> = Rgb([127, 0, 127]);
const SHAKTURI_VIOLET: Rgb<u8> = Rgb([255, 0, 255]);
const PINK: Rgb<u8> = Rgb([255, 170, 255]);

const AQUAMARINE: Rgb<u8> = Rgb([127, 255, 212]);
const CYAN: Rgb<u8> = Rgb([0, 255, 255]);
const TEAL: Rgb<u8> = Rgb([0, 170, 170]);

const K8_KIWI: Rgb<u8> = Rgb([192, 248, 098]);
const K8_TEAL: Rgb<u8> = Rgb([016, 166, 144]);
const K8_NAVY: Rgb<u8> = Rgb([017, 102, 163]);
const K8_MIDNIGHT: Rgb<u8> = Rgb([061, 073, 135]);
const K8_SHAKURAS: Rgb<u8> = Rgb([064, 043, 109]);

// Color themes
pub fn grayscale_theme() -> Vec<Rgb<u8>> {
    return vec![BLACK, WHITE];
}

pub fn fire_theme() -> Vec<Rgb<u8>> {
    return vec![RED, ORANGE, YELLOW];
}

pub fn water_theme() -> Vec<Rgb<u8>> {
    return vec![CYAN, AQUAMARINE, WHITE];
}

pub fn k8_peacock() -> Vec<Rgb<u8>> {
    return vec![
        K8_KIWI,
        K8_TEAL,
        K8_NAVY,
        K8_MIDNIGHT,
        K8_SHAKURAS
    ];
}

/// Translates `a_channel` toward `b_channel` by `degree` percent
fn blend_color_channel(a_channel: u8, b_channel: u8, degree: f64) -> u8 {
    if degree == 0.0 {
        return a_channel;
    }
    else if degree == 1.0 {
        return b_channel;
    }
    else if a_channel == b_channel {
        return a_channel;
    }
    else {
        let delta: u8 = (a_channel as i32 - b_channel as i32).abs() as u8;
        let blended_delta: u8 = (delta as f64 * degree).round() as u8;
        let blended_a: u8;

        if a_channel <= b_channel {
            blended_a = (a_channel + blended_delta) as u8;
        }
        else {
            blended_a = (a_channel - blended_delta) as u8;
        }

        return blended_a;
    }
}

#[test]
fn test_blend_color_channel() {
    assert_eq!(blend_color_channel(255, 0, 0.5), 127);
    assert_eq!(blend_color_channel(0, 255, 0.5), 128);

    assert_eq!(blend_color_channel(0, 100, 0.5), 50);
    assert_eq!(blend_color_channel(0, 100, 0.01), 1);
    assert_eq!(blend_color_channel(0, 100, 0.99), 99);

    assert_eq!(blend_color_channel(100, 0, 0.5), 50);
    assert_eq!(blend_color_channel(100, 0, 0.01), 99);
    assert_eq!(blend_color_channel(100, 0, 0.99), 1);

    assert_eq!(blend_color_channel(13, 26, 0.76), 23);

    assert_eq!(blend_color_channel(255, 0, 0.25), 191);
    assert_eq!(blend_color_channel(255, 0, 0.75), 64);
    assert_eq!(blend_color_channel(0, 255, 0.25), 64);
    assert_eq!(blend_color_channel(0, 255, 0.75), 191);

    assert_eq!(blend_color_channel(255, 0, 0.0), 255);
    assert_eq!(blend_color_channel(255, 0, 1.0), 0);
    assert_eq!(blend_color_channel(0, 255, 0.0), 0);
    assert_eq!(blend_color_channel(0, 255, 1.0), 255);

    assert_eq!(blend_color_channel(7, 7, 0.0), 7);
    assert_eq!(blend_color_channel(7, 7, 0.5), 7);
    assert_eq!(blend_color_channel(7, 7, 1.0), 7);
}

/// Returns a color made by blending the color `a` into color `b` by the given
/// `degree`
///
/// For example, if `a` is black and `b` is white and `degree` is 0.5, this
/// returns perfectly-balanced gray. If `degree` is 1.0, this returns white.
fn blend_colors(a: &Rgb<u8>, b: &Rgb<u8>, degree: f64) -> Rgb<u8> {
    let r = blend_color_channel(a[0], b[0], degree);
    let g = blend_color_channel(a[1], b[1], degree);
    let b = blend_color_channel(a[2], b[2], degree);

    return Rgb([r,g,b]);
}

#[test]
fn test_blend_colors() {
    assert_eq!(
        blend_colors(
            &BLACK,
            &WHITE,
            0.5
        ),
        Rgb([128, 128, 128])
    );

    assert_eq!(
        blend_colors(
            &BLACK,
            &WHITE,
            1.0
        ),
        WHITE
    );

    assert_eq!(
        blend_colors(
            &BLACK,
            &WHITE,
            0.0
        ),
        BLACK
    );

    assert_eq!(
        blend_colors(
            &RED,
            &RED,
            0.0
        ),
        RED
    );

    assert_eq!(
        blend_colors(
            &RED,
            &RED,
            1.0
        ),
        RED
    );

    assert_eq!(
        blend_colors(
            &RED,
            &ORANGE,
            0.0
        ),
        RED
    );

    assert_eq!(
        blend_colors(
            &RED,
            &ORANGE,
            0.25
        ),
        Rgb([255, 32, 0])
    );

    assert_eq!(
        blend_colors(
            &RED,
            &ORANGE,
            1.0
        ),
        ORANGE
    );

    assert_eq!(
        blend_colors(
            &YELLOW,
            &RED,
            0.63
        ),
        Rgb([255, 94, 0])
    );
}

/// Returns the color in `palette` that maps onto `iterations`
///
/// When `iterations` is equal to `limit`, this always returns black.
pub fn iterations_to_color(
    iterations: u32,
    limit: u32,
    palette: &Vec<Rgb<u8>>
) -> Rgb<u8> {
    assert!(palette.len() > 1); // We need at least 2 colors
    
    if iterations == limit {
        return BLACK;
    }

    /*
    For a visualization of what the subranges below are, and why there's always
    1-less-than-colors of them, consider:

    colors = [RED, GREEN, BLUE, WHITE]
                └┬─┘   └┬─┘  └┬─┘
                 │      │     │
            ┌────┴──────┴─┐ ┌─┴───────────┐ ┌─────────────┐
            │RED     GREEN├─┤GREEN    BLUE├─┤BLUE    WHITE│
            │0%.......100%│ │0%.......100%│ │0%.......100%│
            └─────────────┘ └─────────────┘ └─────────────┘
            Subrange 0      Subrange 1      Subrange 2

    With those 4 colors, there are 3 subranges, each defining a gradient between
    a pair of two adjacent colors.

    The colors form the "range", which is broken into "subranges".
    More elaborately,

    Range coverage
    0%...............25%.............50%...............75%............100%
    │                │               │                 │              │
    ├────────────────┼───────────────┼─────────────────┼──────────────┤
    │RED R r r    g g G GREEN G g g     b b B BLUE B b b   w w W WHITE│ Range
    ├─────────────────────┼─────────────────────┼─────────────────────┤
    │                     │                     │                     │
    0%....................│33%..................│66%..................100%
    Range coverage        │                     │
                          │                     │
    ┌─────────────────────┼─────────────────────┼─────────────────────┐
    │RED R r     g G GREEN│GREEN G g    b B BLUE│BLUE B b    w W WHITE│
    ├─────────────────────┼─────────────────────┴─────────────────────┘
    │     Subrange 0      │     Subrange 1            Subrange 2
    0%....................100%
    Subrange 0 coverage
    */
    
    // Find the subrange we should be in
    let subranges = palette.len() - 1;
    let subrange_width: u32 = (limit as f64 / subranges as f64).round() as u32;
    let mut chosen_subrange = 0;

    if subranges > 1 {
        let mut range_cover = iterations as i32;

        for s in 0..=subranges {
            range_cover -= subrange_width as i32;

            if (range_cover <= 0) || (s == (subranges - 1)) {
                chosen_subrange = s;
                break;
            }
        }
    }

    let start_color: usize = chosen_subrange;
    let next_color: usize = start_color + 1;

    // Find our subrange coverage
    let subrange_cover: f64;

    if iterations > subrange_width {
        // Must use mod to map range cover to subrange cover
        let mapped_iterations = iterations % subrange_width;

        // At subrange boundaries, which are points of pure color, iterations
        // is a multiple of subrange_width.
        // This causes the mod above to yield 0, and the color selection will
        // "lag" behind by one. Bumping the subrange coverage to 100% is
        // equivalent to bumping the color selection forward by one.
        if mapped_iterations == 0 {
            subrange_cover = 1.0;
        }
        else {
            subrange_cover = mapped_iterations as f64 / subrange_width as f64;
        }
    }
    else {
        subrange_cover = iterations as f64 / subrange_width as f64;
    }

    // Return color in subrange gradient
    return blend_colors(&palette[start_color], &palette[next_color], subrange_cover);
}

#[test]
/// Test with a continuous spectrum of three blended colors
fn test_iterations_to_color_odd_spectrum() {
    let palette: Vec<Rgb<u8>> = vec![RED, GREEN, BLUE];
    let width: u32 = 101;
    let height: u32 = 25;
    let limit = 100;

    let mut output_gradient: Vec<Rgb<u8>> = Vec::with_capacity(width as usize);

    for i in 0..width {
        output_gradient.push(iterations_to_color(i, limit, &palette));
    }

    assert!(output_gradient.len() == width as usize);

    // Test the two subranges' boundaries
    assert_eq!(output_gradient[0], RED); // 0/2
    assert_eq!(output_gradient[50], GREEN); // 1/2
    assert_eq!(output_gradient[99], Rgb([0, 5, 250])); // almost 2/2; almost BLUE
    assert_eq!(output_gradient[100], BLACK); // 2/2

    // Test random points in subranges
    assert_eq!(output_gradient[4], Rgb([235, 20, 0]));
    assert_eq!(output_gradient[25], Rgb([127, 128, 0]));
    assert_eq!(output_gradient[39], Rgb([56, 199, 0]));

    assert_eq!(output_gradient[55], Rgb([0, 229, 26]));
    assert_eq!(output_gradient[75], Rgb([0, 127, 128]));
    assert_eq!(output_gradient[90], Rgb([0, 51, 204]));

    // Write blended spectrum to file
    let mut output_image = image::RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            *output_image.get_pixel_mut(x, y) = output_gradient[x as usize];
        }
    }

    output_image.save("test_gradient_odd.png")
        .expect("error writing to image file");
}

#[test]
/// Test with a continuous spectrum of four blended colors
fn test_iterations_to_color_even_spectrum() {
    let palette: Vec<Rgb<u8>> = vec![RED, ORANGE, YELLOW, WHITE];
    let width: u32 = 101;
    let height: u32 = 25;
    let limit = 100;
    
    let mut output_gradient: Vec<Rgb<u8>> = Vec::with_capacity(width as usize);

    for i in 0..width {
        output_gradient.push(iterations_to_color(i, limit, &palette));
    }
    
    assert_eq!(output_gradient.len(), width as usize);

    // Test the three subranges' boundaries
    assert_eq!(output_gradient[0], RED); // 0/3
    assert_eq!(output_gradient[33], ORANGE); // 1/3
    assert_eq!(output_gradient[66], YELLOW); // 2/3
    assert_eq!(output_gradient[99], WHITE); // 2/3
    assert_eq!(output_gradient[100], BLACK); // 3/3

    // Test random points in subranges
    assert_eq!(output_gradient[4], Rgb([255, 15, 0]));
    assert_eq!(output_gradient[16], Rgb([255, 62, 0]));
    assert_eq!(output_gradient[25], Rgb([255, 96, 0]));

    assert_eq!(output_gradient[40], Rgb([255, 154, 0]));
    assert_eq!(output_gradient[50], Rgb([255, 193, 0]));
    assert_eq!(output_gradient[61], Rgb([255, 236, 0]));

    assert_eq!(output_gradient[67], Rgb([255, 255, 8]));
    assert_eq!(output_gradient[75], Rgb([255, 255, 70]));
    assert_eq!(output_gradient[85], Rgb([255, 255, 147]));

    // Write blended spectrum to file
    let mut output_image = image::RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            *output_image.get_pixel_mut(x, y) = output_gradient[x as usize];
        }
    }

    output_image.save("test_gradient_even.png")
        .expect("error writing to image file");
}
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

/// Returns a color made by blending the color `a` into color `b` by the given
/// `degree`
///
/// For example, if `a` is black and `b` is white and `degree` is 0.5, this
/// returns perfectly-balanced gray. If `degree` is 1.0, this returns white.
fn blend_colors(a: &Rgb<u8>, b: &Rgb<u8>, degree: f64) -> Rgb<u8> {
    let a_r = a[0];
    let a_g = a[1];
    let a_b = a[2];

    let b_r = b[0];
    let b_g = b[1];
    let b_b = b[2];

    let r: u8;
    let g: u8;
    let b: u8;

    if degree == 0.0 {
        r = a_r;
        g = a_g;
        b = a_b;
    }
    else if degree == 1.0 {
        r = b_r;
        g = b_g;
        b = b_b;
    }
    else {
        r = ((a_r as i16 + (((a_r as i16 - b_r as i16).abs() as f64 * degree) as i16)) % 255) as u8;
        g = ((a_g as i16 + (((a_g as i16 - b_g as i16).abs() as f64 * degree) as i16)) % 255) as u8;
        b = ((a_b as i16 + (((a_b as i16 - b_b as i16).abs() as f64 * degree) as i16)) % 255) as u8;
    }

    return Rgb([r,g,b]);
}

#[test]
fn test_blend_colors() {
    assert_eq!(blend_colors(&Rgb([0, 0, 0]),
                            &Rgb([255, 255, 255]),
                            0.5),
               Rgb([127, 127, 127]));

    assert_eq!(blend_colors(&Rgb([0, 0, 0]),
                            &Rgb([255, 255, 255]),
                            1.0),
               Rgb([255, 255, 255]));

    assert_eq!(blend_colors(&Rgb([0, 0, 0]),
                            &Rgb([255, 255, 255]),
                            0.0),
               Rgb([0, 0, 0]));

    assert_eq!(blend_colors(&Rgb([255, 0, 0]),
                            &Rgb([255, 0, 0]),
                            0.0),
               Rgb([255, 0, 0]));

    assert_eq!(blend_colors(&Rgb([255, 0, 0]),
                            &Rgb([255, 0, 0]),
                            1.0),
               Rgb([255, 0, 0]));

    assert_eq!(blend_colors(&YELLOW,
                            &RED,
                            0.63),
               Rgb([0, 160, 0]));
}

/// Returns the color that maps onto `escape_time` from `palette`, as influenced
/// by `flux`
///
/// `flux` is an arbitrary number that controls how quickly colors change.
/// There's no straightforward relationship with how small or large it is; it
/// just serves as a consistent way to get a useful quotient in this function.
pub fn color_map(escape_time: u32,
                 palette: &Vec<Rgb<u8>>,
                 flux: u32) -> Rgb<u8> {
    assert!(palette.len() > 1); // We need at least 2 colors

    let start_color: Rgb<u8> = palette[escape_time as usize % palette.len()];
    let next_color: Rgb<u8> = palette[(escape_time + 1) as usize % palette.len()];
    let degree: f64 = (escape_time % flux) as f64 / flux as f64; // How much to blend colors

    return blend_colors(&start_color, &next_color, degree);
}

#[test]
fn test_color_map() {
    let palette: Vec<Rgb<u8>> = vec![RED, ORANGE, YELLOW];

    assert_eq!(color_map(563, &palette, 100), Rgb([0, 160, 0]));
}

//! Module containing various utility functions.


use std::iter;
use image::Rgb;
use std::ops::Index;


/// Magic number used for determining whether an image is a BMP.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
pub static BMP_MAGIC: &[u8] = &[0x42, 0x4D];

/// Magic number used for determining whether an image is an ICO.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
pub static ICO_MAGIC: &[u8] = &[0x00, 0x00, 0x01, 0x00];

/// Magic number used for determining whether an image is a GIF.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
pub static GIF_MAGIC: &[u8] = &[0x47, 0x49, 0x46, 0x38];

/// Magic number used for determining whether an image is a PNG.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
pub static PNG_MAGIC: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

/// Magic number used for determining whether an image is a JPEG.
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/List_of_file_signatures).
pub static JPEG_MAGIC: &[u8] = &[0xFF, 0xD8, 0xFF, 0xE0];


/// ANSI colours for a white-background terminal, in the same order as `ANSI_COLOUR_ESCAPES`.
///
/// Acquired from screenshot provided by [@Ell](https://github.com/elliotpotts):
///
/// ![Terminal screenshot](https://cloud.githubusercontent.com/assets/6709544/18532811/e7e87a6e-7ade-11e6-868f-f6d2f9faec27.png)
pub static ANSI_COLOURS_WHITE_BG: [Rgb<u8>; 16] = [Rgb([0xEE, 0xE8, 0xD5]),
                                                   Rgb([0xDC, 0x32, 0x2F]),
                                                   Rgb([0x85, 0x99, 0x00]),
                                                   Rgb([0xB5, 0x89, 0x00]),
                                                   Rgb([0x26, 0x8B, 0xD2]),
                                                   Rgb([0xD3, 0x36, 0x82]),
                                                   Rgb([0x2A, 0xA1, 0x98]),
                                                   Rgb([0x07, 0x36, 0x42]),

                                                   Rgb([0xFD, 0xF6, 0xE3]),
                                                   Rgb([0xCB, 0x4B, 0x16]),
                                                   Rgb([0x93, 0xA1, 0xA1]),
                                                   Rgb([0x83, 0x94, 0x96]),
                                                   Rgb([0x65, 0x7B, 0x83]),
                                                   Rgb([0x6C, 0x71, 0xC4]),
                                                   Rgb([0x58, 0x6E, 0x75]),
                                                   Rgb([0x00, 0x2B, 0x36])];

/// Linux-theme ANSI colours, in the same order as `ANSI_COLOUR_ESCAPES`.
///
/// Acquired from the [`colorname` table in st](https://git.suckless.org/st/file/config.def.h.html#l86),
/// as decoded according to the [X11 colour names Wikipedia article](https://en.wikipedia.org/wiki/X11_color_names):
///
/// ```c
/// static const char *colorname[] = {
///     /* 8 normal colors */
///     "black",
///     "red3",
///     "green3",
///     "yellow3",
///     "blue2",
///     "magenta3",
///     "cyan3",
///     "gray90",
///
///     /* 8 bright colors */
///     "gray50",
///     "red",
///     "green",
///     "yellow",
///     "#5c5cff",
///     "magenta",
///     "cyan",
///     "white",
///
///     // …
/// }
/// ```
pub static ANSI_COLOURS_BLACK_BG: [Rgb<u8>; 16] = [Rgb([0x00, 0x00, 0x00]),
                                                   Rgb([0xCD, 0x00, 0x00]),
                                                   Rgb([0x00, 0xCD, 0x00]),
                                                   Rgb([0xCD, 0xCD, 0x00]),
                                                   Rgb([0x00, 0x00, 0xEE]),
                                                   Rgb([0xCD, 0x00, 0xCD]),
                                                   Rgb([0x00, 0xCD, 0xCD]),
                                                   Rgb([0xE6, 0xE6, 0xE6]),

                                                   Rgb([0x80, 0x80, 0x80]),
                                                   Rgb([0xFF, 0x00, 0x00]),
                                                   Rgb([0x00, 0xFF, 0x00]),
                                                   Rgb([0xFF, 0xFF, 0x00]),
                                                   Rgb([0x5C, 0x5C, 0xFF]),
                                                   Rgb([0xFF, 0x00, 0xFF]),
                                                   Rgb([0x00, 0xFF, 0xFF]),
                                                   Rgb([0xFF, 0xFF, 0xFF])];

/// ANSI background colour escapes.
pub static ANSI_COLOUR_ESCAPES: [&str; 16] = ["\x1B[0;30m",
                                              "\x1B[0;31m",
                                              "\x1B[0;32m",
                                              "\x1B[0;33m",
                                              "\x1B[0;34m",
                                              "\x1B[0;35m",
                                              "\x1B[0;36m",
                                              "\x1B[0;37m",
                                              "\x1B[1;30m",
                                              "\x1B[1;31m",
                                              "\x1B[1;32m",
                                              "\x1B[1;33m",
                                              "\x1B[1;34m",
                                              "\x1B[1;35m",
                                              "\x1B[1;36m",
                                              "\x1B[1;37m"];

/// ANSI background colour escapes.
pub static ANSI_BG_COLOUR_ESCAPES: [&str; 8] = ["\x1B[40m", "\x1B[41m", "\x1B[42m", "\x1B[43m", "\x1B[44m", "\x1B[45m", "\x1B[46m", "\x1B[47m"];

/// Reset ANSI attributes
pub static ANSI_RESET_ATTRIBUTES: &str = "\x1B[0m";


/// Create a string consisting of `n` repetitions of `what`.
///
/// # Examples
///
/// ```
/// # use termimage::util::mul_str;
/// assert_eq!(mul_str("Го! ", 3), "Го! Го! Го! ".to_string());
/// ```
pub fn mul_str(what: &str, n: usize) -> String {
    iter::repeat(what).take(n).collect()
}

/// Get the closest colour to the provided one out of the specified list of colours and retirn its index.
///
/// The formula is the last one from the
/// [Euclidean section in the Color difference article on Wikipedia](https://en.wikipedia.org/wiki/Color_difference#Euclidean)
pub fn closest_colour<P: Index<usize, Output = u8>>(to: Rgb<u8>, out_of: &[P]) -> usize {
    let mut diffs = out_of.iter()
        .enumerate()
        .map(|(i, rgb)| {
            let r = (rgb[0] as f32 + to[0] as f32) / 2.0;
            ((2.0 + (r / 256.0)) * (rgb[0] as f32 - to[0] as f32).powi(2) + 4.0 * (rgb[1] as f32 - to[1] as f32).powi(2) +
             (2.0 + ((255.0 - r) / 256.0)) * (rgb[2] as f32 - to[2] as f32).powi(2),
             i)
        })
        .collect::<Vec<_>>();
    diffs.sort_by(|&(lhs_diff, _), &(rhs_diff, _)| lhs_diff.partial_cmp(&rhs_diff).unwrap());
    diffs[0].1
}

/// Get the background colour set for the specified foregournd colour set
#[inline(always)]
pub fn bg_colours_for<C: Index<usize, Output = u8>>(foreground_colours: &[C]) -> &[C] {
    &foreground_colours[0..8]
}

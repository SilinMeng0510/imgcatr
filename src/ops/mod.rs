//! Main functions doing actual work.
//!
//! Use `guess_format()` to get the image format from a path,
//! then read the image using `load_image()` to the size given by `image_resized_size()`,
//! resize it to terminal size with `resize_image()`
//! and display it with `write_[no_]ansi[_truecolor]()`,
//! or display it yourself with approximations from `create_colourtable()`.


use self::super::util::{ANSI_BG_COLOUR_ESCAPES, ANSI_RESET_ATTRIBUTES, ANSI_COLOUR_ESCAPES, JPEG_MAGIC, BMP_MAGIC, ICO_MAGIC, GIF_MAGIC, PNG_MAGIC,
                        closest_colour, bg_colours_for};
use image::{self, GenericImageView, DynamicImage, ImageFormat, Pixel};
use std::io::{BufReader, Write, Read};
use image::imageops::FilterType;
use self::super::Error;
use std::path::PathBuf;
use std::ops::Index;
use std::fs::File;

mod no_ansi;

pub use self::no_ansi::write_no_ansi;


/// Guess the image format from its extension or magic.
///
/// # Examples
///
/// Correct:
///
/// ```
/// # extern crate image;
/// # extern crate termimage;
/// # use image::ImageFormat;
/// # use std::path::PathBuf;
/// # use termimage::ops::guess_format;
/// # fn main() {
/// assert_eq!(guess_format(&(String::new(), PathBuf::from("img.png"))), Ok(ImageFormat::Png));
/// assert_eq!(guess_format(&(String::new(), PathBuf::from("img.jpg"))), Ok(ImageFormat::Jpeg));
/// assert_eq!(guess_format(&(String::new(), PathBuf::from("img.gif"))), Ok(ImageFormat::Gif));
/// assert_eq!(guess_format(&(String::new(), PathBuf::from("img.bmp"))), Ok(ImageFormat::Bmp));
/// assert_eq!(guess_format(&(String::new(), PathBuf::from("img.ico"))), Ok(ImageFormat::Ico));
/// # }
/// ```
///
/// Incorrect:
///
/// ```
/// # use std::path::PathBuf;
/// # use termimage::Error;
/// # use termimage::ops::guess_format;
/// assert_eq!(guess_format(&("src/ops.rs".to_string(), PathBuf::from("src/ops/mod.rs"))),
/// Err(Error::GuessingFormatFailed("src/ops.rs".to_string())));
/// ```
pub fn guess_format(file: &(String, PathBuf)) -> Result<ImageFormat, Error> {
    file.1
        .extension()
        .and_then(|ext| match &ext.to_str().unwrap().to_lowercase()[..] {
            "png" => Some(Ok(ImageFormat::Png)),
            "jpg" | "jpeg" | "jpe" | "jif" | "jfif" | "jfi" => Some(Ok(ImageFormat::Jpeg)),
            "gif" => Some(Ok(ImageFormat::Gif)),
            "webp" => Some(Ok(ImageFormat::WebP)),
            "ppm" => Some(Ok(ImageFormat::Pnm)),
            "tiff" | "tif" => Some(Ok(ImageFormat::Tiff)),
            "tga" => Some(Ok(ImageFormat::Tga)),
            "bmp" | "dib" => Some(Ok(ImageFormat::Bmp)),
            "ico" => Some(Ok(ImageFormat::Ico)),
            "hdr" => Some(Ok(ImageFormat::Hdr)),
            _ => None,
        })
        .unwrap_or_else(|| {
            let mut buf = [0; 32];
            let read = File::open(&file.1).map_err(|_| Error::OpeningImageFailed(file.0.clone()))?.read(&mut buf).unwrap();
            let buf = &buf[..read];

            if buf.len() >= PNG_MAGIC.len() && &buf[..PNG_MAGIC.len()] == PNG_MAGIC {
                Ok(ImageFormat::Png)
            } else if buf.len() >= JPEG_MAGIC.len() && &buf[..JPEG_MAGIC.len()] == JPEG_MAGIC {
                Ok(ImageFormat::Jpeg)
            } else if buf.len() >= GIF_MAGIC.len() && &buf[..GIF_MAGIC.len()] == GIF_MAGIC {
                Ok(ImageFormat::Gif)
            } else if buf.len() >= BMP_MAGIC.len() && &buf[..BMP_MAGIC.len()] == BMP_MAGIC {
                Ok(ImageFormat::Bmp)
            } else if buf.len() >= ICO_MAGIC.len() && &buf[..ICO_MAGIC.len()] == ICO_MAGIC {
                Ok(ImageFormat::Ico)
            } else {
                Err(Error::GuessingFormatFailed(file.0.clone()))
            }
        })
}

/// Load an image from the specified file as the specified format.
///
/// Get the image fromat with `guess_format()`.
pub fn load_image(file: &(String, PathBuf), format: ImageFormat) -> Result<DynamicImage, Error> {
    Ok(image::load(BufReader::new(File::open(&file.1).map_err(|_| Error::OpeningImageFailed(file.0.clone()))?),
                   format)
        .unwrap())
}

/// Get the image size to downscale to, given its size, the terminal's size and whether to preserve its aspect.
///
/// The resulting image size is twice as tall as the terminal size because we print two pixels per cell (height-wise).
pub fn image_resized_size(size: (u32, u32), term_size: (u32, u32), preserve_aspect: bool) -> (u32, u32) {
    if !preserve_aspect {
        return (term_size.0, term_size.1 * 2);
    }

    let nwidth = term_size.0;
    let nheight = term_size.1 * 2;
    let (width, height) = size;

    let ratio = width as f32 / height as f32;
    let nratio = nwidth as f32 / nheight as f32;

    let scale = if nratio > ratio {
        nheight as f32 / height as f32
    } else {
        nwidth as f32 / width as f32
    };

    ((width as f32 * scale) as u32, (height as f32 * scale) as u32)
}

/// Resize the specified image to the specified size.
pub fn resize_image(img: &DynamicImage, size: (u32, u32)) -> DynamicImage {
    img.resize_exact(size.0, size.1, FilterType::Nearest)
}

/// Create a line-major table of (upper, lower) colour approximation indices given the supported colours therefor.
///
/// # Examples
///
/// Approximate `img` to ANSI and display it to stdout.
///
/// ```
/// # extern crate termimage;
/// # extern crate image;
/// # use termimage::util::{ANSI_COLOURS_WHITE_BG, ANSI_COLOUR_ESCAPES, ANSI_BG_COLOUR_ESCAPES, bg_colours_for};
/// # use termimage::ops::create_colourtable;
/// # fn main() {
/// # let img = image::DynamicImage::new_rgb8(16, 16);
/// for line in create_colourtable(&img, &ANSI_COLOURS_WHITE_BG, &bg_colours_for(&ANSI_COLOURS_WHITE_BG)) {
///     for (upper_clr, lower_clr) in line {
///         print!("{}{}\u{2580}", // ▀
///                ANSI_COLOUR_ESCAPES[upper_clr],
///                ANSI_BG_COLOUR_ESCAPES[lower_clr]);
///     }
///     println!("{}{}", ANSI_COLOUR_ESCAPES[15], ANSI_BG_COLOUR_ESCAPES[0]);
/// }
/// # }
/// ```
pub fn create_colourtable<C: Index<usize, Output = u8>>(img: &DynamicImage, upper_colours: &[C], lower_colours: &[C]) -> Vec<Vec<(usize, usize)>> {
    let (width, height) = img.dimensions();
    let term_h = height / 2;

    (0..term_h)
        .map(|y| {
            let upper_y = y * 2;
            let lower_y = upper_y + 1;

            (0..width)
                .map(|x| (closest_colour(img.get_pixel(x, upper_y).to_rgb(), upper_colours), closest_colour(img.get_pixel(x, lower_y).to_rgb(), lower_colours)))
                .collect()
        })
        .collect()
}

/// Display the specified image approximating it to the specified colours in the default console using ANSI escape codes.
pub fn write_ansi<W: Write, C: Index<usize, Output = u8>>(out: &mut W, img: &DynamicImage, foreground_colours: &[C]) {
    for line in create_colourtable(img, foreground_colours, &bg_colours_for(foreground_colours)) {
        for (upper_clr, lower_clr) in line {
            write!(out,
                   "{}{}\u{2580}", // ▀
                   ANSI_COLOUR_ESCAPES[upper_clr],
                   ANSI_BG_COLOUR_ESCAPES[lower_clr])
                .unwrap();
        }
        writeln!(out, "{}", ANSI_RESET_ATTRIBUTES).unwrap();
    }
}

/// Display the specified image in the default console using ANSI 24-bit escape colour codes.
pub fn write_ansi_truecolor<W: Write>(out: &mut W, img: &DynamicImage) {
    let (width, height) = img.dimensions();
    let term_h = height / 2;

    for y in 0..term_h {
        let upper_y = y * 2;
        let lower_y = upper_y + 1;

        for x in 0..width {
            let upper_pixel = img.get_pixel(x, upper_y).to_rgb();
            let lower_pixel = img.get_pixel(x, lower_y).to_rgb();

            write!(out,
                   "\x1B[38;2;{};{};{}m\
                    \x1B[48;2;{};{};{}m\u{2580}", // ▀
                   upper_pixel[0],
                   upper_pixel[1],
                   upper_pixel[2],
                   lower_pixel[0],
                   lower_pixel[1],
                   lower_pixel[2])
                .unwrap();
        }
        writeln!(out, "{}", ANSI_RESET_ATTRIBUTES).unwrap();
    }
}

/// Display the specified image in the ascii art style with specified scale.
pub fn write_ascii(img: &DynamicImage) {
    println!("{:?}", img.dimensions());
    let (width,height) = img.dimensions();
    for y in 0..height{
        for x in 0..width{
            if y % 2 == 0 && x % 1 == 0{
                let pix = img.get_pixel(x,y);
                let mut intent = pix[0]/3 + pix[1]/3 + pix[2]/3;
                if pix[3] ==0{
                    intent = 0;
                }
                print!("{}",get_str_ascii(intent));
            } 
        }
        if y % 2 ==0{
            println!("");
        }
    }
}

fn get_str_ascii(intent :u8)-> &'static str{
    let index = intent/32;
    let ascii = [" ",".",",","-","~","+","=","@"];
    return ascii[index as usize];
}


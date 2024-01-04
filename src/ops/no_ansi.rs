pub use image::DynamicImage;
#[cfg(target_os = "windows")]
use self::imports::*;

#[cfg(target_os = "windows")]
mod imports {
    pub use winapi::um::wincon::{CONSOLE_SCREEN_BUFFER_INFOEX, SMALL_RECT, COORD, GetConsoleScreenBufferInfoEx, FillConsoleOutputAttribute};
    pub use self::super::super::super::util::{closest_colour, mul_str};
    pub use winapi::um::winbase::STD_OUTPUT_HANDLE;
    pub use self::super::super::create_colourtable;
    pub use image::{GenericImageView, Pixel, Rgb};
    pub use winapi::um::processenv::GetStdHandle;
    pub use std::mem;
}


/// Display the specified image in the default console using WinAPI.
#[cfg(target_os = "windows")]
pub fn write_no_ansi(img: &DynamicImage) {
    let (width, height) = img.dimensions();
    let term_h = height / 2;
    print!("{}", mul_str(&format!("{}\n", mul_str("\u{2580}", width as usize)), term_h as usize)); // ▀

    let console_h = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    let mut console_info = CONSOLE_SCREEN_BUFFER_INFOEX {
        cbSize: mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() as u32,
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: 0,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 },
        wPopupAttributes: 0,
        bFullscreenSupported: 0,
        ColorTable: [0; 16],
    };
    unsafe { GetConsoleScreenBufferInfoEx(console_h, &mut console_info) };
    let colours =
        console_info.ColorTable.iter().map(|cr| Rgb([(cr & 0xFF) as u8, ((cr & 0xFF00) >> 8) as u8, ((cr & 0xFF0000) >> 16) as u8])).collect::<Vec<_>>();

    for (y, line) in create_colourtable(img, &colours, &colours).into_iter().enumerate() {
        for (x, (upper_clr, lower_clr)) in line.into_iter().enumerate() {
            unsafe {
                FillConsoleOutputAttribute(console_h,
                                           (console_info.wAttributes & 0xFF00) | ((lower_clr as u16) << 4) | (upper_clr as u16),
                                           1,
                                           COORD {
                                               X: x as i16,
                                               Y: console_info.dwCursorPosition.Y - (term_h as i16 - y as i16),
                                           },
                                           &mut 0);
            }
        }
    }
}

/// Display the specified image in the default console using WinAPI.
///
/// Or, actually, don't. This is Linux, after all...
#[cfg(not(target_os = "windows"))]
pub fn write_no_ansi(_: &DynamicImage) {}

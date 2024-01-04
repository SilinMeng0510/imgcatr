#![allow(unused_imports)]
use std::io::{BufWriter, Write, stdout, stderr};
use std::process::exit;
use image::GenericImageView;

fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        err.print_error(&mut stderr());
        err.exit_value()
    } else {
        0
    }
}

fn result_main() -> Result<(), imgcatr::Error> {
    let opts = imgcatr::Options::parse();

    let format = imgcatr::ops::guess_format(&opts.image)?;
    let img = imgcatr::ops::load_image(&opts.image, format)?;

    let img_s = imgcatr::ops::image_resized_size(img.dimensions(), opts.size, opts.preserve_aspect);
    let resized = imgcatr::ops::resize_image(&img, img_s);

    match opts.ansi_out {
        Some(ansi) => {
            let mut out = BufWriter::new(stdout().lock());
            match ansi {
                imgcatr::AnsiOutputFormat::Truecolor => imgcatr::ops::write_ansi_truecolor(&mut out, &resized),
                imgcatr::AnsiOutputFormat::SimpleWhite => imgcatr::ops::write_ansi(&mut out, &resized, &imgcatr::util::ANSI_COLOURS_WHITE_BG),
                imgcatr::AnsiOutputFormat::SimpleBlack => imgcatr::ops::write_ansi(&mut out, &resized, &imgcatr::util::ANSI_COLOURS_BLACK_BG),
                imgcatr::AnsiOutputFormat::ASCII => imgcatr::ops::write_ascii(&resized),
            }
            out.flush().unwrap();
        }
        None => imgcatr::ops::write_no_ansi(&resized),
    }

    Ok(())
}

//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use termimage::Options;
//! let options = Options::parse();
//! println!("Image to display: {}", options.image.0);
//! ```

use clap::{command, Arg};
use std::path::PathBuf;
// use std::str::FromStr;
use term_size;
use std::fs;


/// Supported ANSI output formats
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnsiOutputFormat {
    /// Truecolor ANSI 24-bit colour
    Truecolor,
    /// Dumb ANSI 3-bit colour, for black backgrounds
    SimpleBlack,
    /// Dumb ANSI 3-bit colour, for white backgrounds
    SimpleWhite,
    /// ASCII Art
    ASCII,
}


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Image file to display.
    /// This tuple contains the plaintext name (user-friendly) and a normalised path (programmer-friendly).
    pub image: (String, PathBuf),
    /// Output size. Default: detected from terminal size or no default.
    pub size: (u32, u32),
    /// Whether to preserve the image's aspect ratio when resizing. Default: `true`.
    pub preserve_aspect: bool,
    /// Whether to output ANSI escapes and in which format. Default: `None` on Windooze when not writing to a file.
    pub ansi_out: Option<AnsiOutputFormat>,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let mut szarg = Arg::new("size")
                            .long("size")
                            .short('s')
                            .value_name("NxM")
                            .value_parser(Options::size_validator)
                            .help("Image size to display");
        let szarg_def;
        let have_dimms = if let Some((w, h)) = term_size::dimensions() {
            szarg_def = format!("{}x{}", w, h - 1);
            szarg = szarg.default_value(&szarg_def);
            true
        } else {
            szarg = szarg.required(true);
            false
        };


        let matches = command!()
            .arg(Arg::new("image")
                .value_name("IMAGE")
                .value_parser(Options::image_file_validator)
                .help("Image file to display")
                .required(true))
            .arg(szarg)
            .arg(Arg::new("force")
                .long("force")
                .short('f')
                .help("Don't preserve the image's aspect ratio")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("ansi")
                .long("ansi")
                .short('a')
                .value_name("ANSI")
                .help("Force output ANSI escape")
                .value_parser(["truecolor", "simple-black", "simple-white", "ascii"]))
            .get_matches();

        let image: &String = matches.get_one::<String>("image").unwrap();
        Options { image: (image.to_string(), fs::canonicalize(image).unwrap()),
                  size: *matches.get_one::<(u32, u32)>("size").unwrap(),
                  preserve_aspect: !matches.get_flag("force"),
                  ansi_out: if cfg!(not(target_os = "windows")) || !have_dimms || matches.contains_id("ansi") {
                    match matches.get_one::<String>("ansi").map(|x| x.as_str()).unwrap_or("truecolor") {
                        "truecolor" => Some(AnsiOutputFormat::Truecolor),
                        "simple-black" => Some(AnsiOutputFormat::SimpleBlack),
                        "simple-white" => Some(AnsiOutputFormat::SimpleWhite),
                        "ascii" => Some(AnsiOutputFormat::ASCII),
                        _ => unreachable!(),
                    }
                } else {
                    None
                },
        }
    }

    fn image_file_validator(s: &str) -> Result<String, String> {
        let path: Result<PathBuf, std::io::Error>  = fs::canonicalize(&s);
        match path {
            Err(_) => Err(format!("Image file \"{}\" not found", s)),
            _ => Ok(s.to_string()),
        }
    }

    fn parse_size(s: &str) -> Option<(u32, u32)> {
        let mut parts = s.splitn(2, |c| c == 'x' || c == 'X');
        let x : u32 =  parts.next()?.parse::<u32>().unwrap();
        let y : u32 = parts.next()?.parse::<u32>().unwrap();
        Some((x, y))
    }
    
    fn size_validator(s: &str) -> Result<(u32, u32), String> {
        match Options::parse_size(&s) {
            None => Err(format!("\"{}\" is not a valid size (in format \"NNNxMMM\")", s)),
            Some((0, _)) | Some((_, 0)) => Err(format!("Can't resize image to size 0")),
            size => Ok(size.unwrap()),
        }
    }
}


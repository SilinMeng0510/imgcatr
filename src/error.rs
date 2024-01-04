use std::io::Write;


/// Enum representing all possible values the application can fail.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    /// Failed to guess the image format.
    GuessingFormatFailed(String),
    /// Failed to open image file.
    OpeningImageFailed(String),
}

impl Error {
    /// Get the executable exit value from an `Error` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use termimage::Error;
    /// # use std::iter::FromIterator;
    /// let mut out = Vec::new();
    /// Error::GuessingFormatFailed("not_image.rs".to_string()).print_error(&mut out);
    /// assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
    ///            "Failed to guess format of \"not_image.rs\".\n".to_string());
    /// ```
    pub fn print_error<W: Write>(&self, err_out: &mut W) {
        match *self {
            Error::GuessingFormatFailed(ref fname) => writeln!(err_out, "Failed to guess format of \"{}\".", fname).unwrap(),
            Error::OpeningImageFailed(ref fname) => writeln!(err_out, "Failed to open image file \"{}\".", fname).unwrap(),
        }
    }

    /// Get the executable exit value from an `Error` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::process::exit;
    /// # use termimage::Error;
    /// assert_eq!(Error::GuessingFormatFailed("".to_string()).exit_value(), 1);
    /// assert_eq!(Error::OpeningImageFailed("".to_string()).exit_value(), 2);
    /// ```
    pub fn exit_value(&self) -> i32 {
        match *self {
            Error::GuessingFormatFailed(_) => 1,
            Error::OpeningImageFailed(_) => 2,
        }
    }
}
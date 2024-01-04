// #[macro_use]
// extern crate clap;

mod error;
mod options;

pub mod ops;
pub mod util;

pub use error::Error;
pub use options::{Options, AnsiOutputFormat};
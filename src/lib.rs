//! Font toolbox.
//!
//! # Example
//!
//! ```
//! extern crate font;
//!
//! use font::File;
//! use font::Operation::*;
//!
//! # fn main() {
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let file = File::open(path).unwrap();
//! let glyph = file[0].draw('&').unwrap().unwrap();
//!
//! for operation in glyph.iter() {
//!     match operation {
//!         &Curve(..) => println!("Curve!"),
//!         &Line(..) => println!("Line!"),
//!         &Move(..) => println!("Move!"),
//!     }
//! }
//! # }
//! ```

/// An error.
pub type Error = std::io::Error;

/// A point.
pub type Point = (f32, f32);

/// A result.
pub type Result<T> = std::io::Result<T>;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

macro_rules! some(
    ($option:expr, $message:expr) => (match $option {
        Some(value) => value,
        _ => raise!($message),
    });
);

mod case;
mod file;
mod font;
mod format;
mod glyph;

pub use case::Case;
pub use file::File;
pub use font::Font;
pub use glyph::{Curve, Glyph, Operation};

//! Parser for TrueType fonts.

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
    ($($argument:tt)+) => (raise!(format!($($argument)+)));
);

mod tape;

pub mod compound;
pub mod primitive;

pub use tape::{Tape, Value};

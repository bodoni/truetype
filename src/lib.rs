//! Parser for TrueType fonts.

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::io::Result<T>;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
    ($($argument:tt)+) => (raise!(format!($($argument)+)));
);

mod tape;

mod compound;
mod primitive;

pub use compound::*;
pub use primitive::*;
pub use tape::{Tape, Value};

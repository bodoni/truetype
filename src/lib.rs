//! Parser for TrueType fonts.

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
    ($($argument:tt)+) => (raise!(format!($($argument)+)));
);

macro_rules! tag(
    ($value:expr) => (unsafe {
        ::std::mem::transmute::<_, [u8; 4]>(u32::from_be(::std::mem::transmute($value)))
    });
);

mod tape;

pub mod compound;
pub mod primitive;

pub use tape::{Tape, Value};

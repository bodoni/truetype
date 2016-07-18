use std::io::{Read, Seek, SeekFrom};

use Result;

/// A type that can read.
pub trait Tape: Read + Seek + Sized {
    #[doc(hidden)]
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    #[doc(hidden)]
    #[inline]
    fn peek<T: Value>(&mut self) -> Result<T> {
        self.stay(|tape| Value::read(tape))
    }

    #[doc(hidden)]
    #[inline]
    fn position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }

    #[doc(hidden)]
    #[inline(always)]
    fn take<T: Value>(&mut self) -> Result<T> {
        Value::read(self)
    }

    #[doc(hidden)]
    fn stay<F, T>(&mut self, mut body: F) -> Result<T> where F: FnMut(&mut Self) -> Result<T> {
        let position = try!(self.position());
        let result = body(self);
        try!(self.jump(position));
        result
    }
}

/// A type that can be read.
pub trait Value: Sized {
    /// Read a value.
    fn read<T: Tape>(&mut T) -> Result<Self>;
}

/// A type that can be read provided a parameter.
pub trait Walue<P>: Sized {
    /// Read a value.
    fn read<T: Tape>(&mut T, P) -> Result<Self>;
}

impl<T: Read + Seek> Tape for T {}

macro_rules! read(
    ($tape:ident, $count:expr, $buffer:ident) => (
        if try!(::std::io::Read::read($tape, &mut $buffer)) != $count {
            return raise!("failed to read as much as needed");
        }
    );
    ($tape:ident, $size:expr) => (unsafe {
        let mut buffer: [u8; $size] = ::std::mem::uninitialized();
        read!($tape, $size, buffer);
        ::std::mem::transmute(buffer)
    });
);

macro_rules! value {
    ($kind:ident, 1) => (impl Value for $kind {
        #[inline]
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            Ok(read!(tape, 1))
        }
    });
    ($kind:ident, $size:expr) => (impl Value for $kind {
        #[inline]
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            Ok($kind::from_be(read!(tape, $size)))
        }
    });
}

value!(i8, 1);
value!(u8, 1);
value!(i16, 2);
value!(u16, 2);
value!(u32, 4);
value!(i64, 8);

macro_rules! value(
    ([i8; $count:expr]) => (impl Value for [i8; $count] {
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            let mut array: [u8; $count] = unsafe { ::std::mem::uninitialized() };
            if try!(::std::io::Read::read(tape, &mut array)) != $count {
                raise!("failed to read as much as needed")
            }
            Ok(unsafe { ::std::mem::transmute(array) })
        }
    });
    ([u8; $count:expr]) => (impl Value for [u8; $count] {
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            let mut array: [u8; $count] = unsafe { ::std::mem::uninitialized() };
            if try!(::std::io::Read::read(tape, &mut array)) != $count {
                raise!("failed to read as much as needed")
            }
            Ok(array)
        }
    });
);

value!([i8; 4]);
value!([u8; 10]);

impl<V> Walue<usize> for Vec<V> where V: Value {
    fn read<T: Tape>(tape: &mut T, count: usize) -> Result<Self> {
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(try!(Value::read(tape)));
        }
        Ok(values)
    }
}

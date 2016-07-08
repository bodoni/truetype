use {Result, Tape, Value};

/// A fixed-point number.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Number(pub u32);

impl From<Number> for f32 {
    #[inline]
    fn from(number: Number) -> Self {
        const SCALE: f32 = 1f32 / (1 << 16) as f32;
        SCALE * (number.0 as f32)
    }
}

impl Value for Number {
    #[inline(always)]
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(Number(try!(Value::read(tape))))
    }
}

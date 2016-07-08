use {Result, Tape, Value};

/// A fixed-point number.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fixed(pub u32);

impl From<Fixed> for f32 {
    #[inline]
    fn from(fixed: Fixed) -> Self {
        const SCALE: f32 = 1f32 / (1 << 16) as f32;
        SCALE * (fixed.0 as f32)
    }
}

impl Value for Fixed {
    #[inline(always)]
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(Fixed(try!(Value::read(tape))))
    }
}

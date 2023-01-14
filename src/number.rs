use crate::{Result, Tape, Value};

macro_rules! implement {
    ($(#[$attribute:meta])* pub $name:ident($kind:ty | $fraction:expr)) => {
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub $kind);

        impl From<$name> for f32 {
            #[inline]
            fn from(number: $name) -> Self {
                const SCALE: f32 = 1f32 / (1 << $fraction) as f32;
                SCALE * (number.0 as f32)
            }
        }

        impl Value for $name {
            #[inline(always)]
            fn read<T: Tape>(tape: &mut T) -> Result<Self> {
                Ok($name(tape.take()?))
            }
        }
    }
}

implement! {
    #[doc = "A fixed-point number in format Q2.14."]
    #[allow(non_camel_case_types)]
    pub q16(u16 | 14)
}

implement! {
    #[doc = "A fixed-point number in format Q16.16."]
    #[allow(non_camel_case_types)]
    pub q32(u32 | 16)
}

#[cfg(test)]
mod tests {
    use super::q16;

    #[test]
    fn from() {
        let cases: Vec<(u16, f32)> = vec![
            (0x7fff, 1.999939),
            (0x7000, 1.75),
            (0x0001, 0.000061),
            (0x0000, 0.0),
            (0xffff, -0.000061),
            (0x8000, -2.0),
        ];
        for (input, output) in cases.into_iter() {
            let input: f32 = q16(input).into();
            assert!((input - output).abs() < 1e-4, "{} != {}", input, output);
        }
    }
}

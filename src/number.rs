use {Result, Tape, Value};

macro_rules! number {
    ($(#[$attribute:meta])* pub $name:ident($kind:ty | $fraction:expr)) => {
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

number! {
    #[doc = "A fixed-point number in format Q2.14."]
    #[allow(non_camel_case_types)]
    pub q16(u16 | 14)
}

number! {
    #[doc = "A fixed-point number in format Q16.16."]
    #[allow(non_camel_case_types)]
    pub q32(u32 | 16)
}

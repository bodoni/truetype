macro_rules! implement {
    ($(#[$attribute:meta])* pub $name:ident($kind:ty | $fraction:literal)) => {
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub $kind);

        impl From<$name> for f32 {
            #[inline]
            fn from(number: $name) -> Self {
                const SCALE: f32 = (1 << $fraction) as f32;
                (number.0 as f32) / SCALE
            }
        }

        impl crate::value::Read for $name {
            #[inline]
            fn read<T: crate::tape::Read>(tape: &mut T) -> $crate::Result<Self> {
                Ok($name(tape.take()?))
            }
        }

        impl crate::value::Write for $name {
            #[inline]
            fn write<T: crate::tape::Write>(&self, tape: &mut T) -> $crate::Result<()> {
                tape.give(&self.0)
            }
        }
    }
}

implement! {
    /// A fixed-point number in format Q2.14.
    #[allow(non_camel_case_types)]
    pub q16(i16 | 14)
}

implement! {
    /// A fixed-point number in format Q16.16.
    #[allow(non_camel_case_types)]
    pub q32(i32 | 16)
}

#[cfg(test)]
mod tests {
    use super::q16;

    #[test]
    fn from() {
        let cases: Vec<(i16, f32)> = vec![
            (0x7fff as i16, 1.999939),
            (0x7000 as i16, 1.75),
            (0x0001 as i16, 0.000061),
            (0x0000 as i16, 0.0),
            (-0x0001 as i16, -0.000061),
            (-0x8000 as i16, -2.0),
        ];
        for (input, output) in cases.into_iter() {
            let input: f32 = q16(input).into();
            assert!((input - output).abs() < 1e-4, "{} != {}", input, output);
        }
    }
}

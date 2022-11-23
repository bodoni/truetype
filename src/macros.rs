#[doc(hidden)]
#[macro_export]
macro_rules! flags {
    ($(#[$attribute:meta])* pub $name:ident($kind:ident) {
        $($mask:expr => $method:ident,)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Copy, Default, Eq, PartialEq)]
        pub struct $name(pub $kind);

        impl $name {
            $(
                #[inline(always)]
                pub fn $method(&self) -> bool {
                    self.0 & $mask > 0
                }
            )*
        }

        impl $crate::Value for $name {
            #[inline(always)]
            fn read<T: $crate::Tape>(tape: &mut T) -> $crate::Result<Self> {
                let value = $name(tape.take::<$kind>()?);
                if value.is_invalid() {
                    raise!("found malformed flags with value {}", value);
                }
                Ok(value)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, concat!(stringify!($name), "(0x{:X})"), self.0)
            }
        }

        impl ::std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(self, formatter)
            }
        }

        impl From<$name> for $kind {
            #[inline(always)]
            fn from(flags: $name) -> $kind {
                flags.0
            }
        }
    );
}

macro_rules! raise(
    (@from $error:ident, $($argument:tt)*) => (
        return Err(
            crate::Error::new(
                ::std::io::ErrorKind::Other,
                crate::ErrorWithSource {
                    description: format!($($argument)*),
                    source: $error,
                },
            )
        )
    );
    ($($argument:tt)*) => (
        return Err(
            crate::Error::new(
                ::std::io::ErrorKind::Other,
                format!($($argument)*),
            )
        )
    );
);

macro_rules! table {
    ($(#[$attribute:meta])* pub $name:ident {
        $($field:ident ($($kind:tt)+) $(= $value:block)* $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $name { $($field ($($kind)+),)* } }
        table! {
            @implement
            pub $name { $($field ($($kind)+) [$($value)*] $(|$($argument),+| $body)*,)* }
        }
    );
    (@define $(#[$attribute:meta])* pub $name:ident { $($field:ident ($kind:ty),)* }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Default)]
        pub struct $name { $(pub $field: $kind,)* }
    );
    (@implement pub $name:ident {
        $($field:ident ($($kind:tt)+) [$($value:block)*] $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl crate::Value for $name {
            fn read<T: crate::Tape>(tape: &mut T) -> crate::Result<Self> {
                let mut table: $name = $name::default();
                $({
                    let value = table!(@read $name, table, tape [$($kind)+] [$($value)*]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@read $name:ident, $this:ident, $tape:ident [$kind:ty] []) => ($tape.take()?);
    (@read $name:ident, $this:ident, $tape:ident [$kind:ty] [$value:block]) => ({
        let value = $tape.take()?;
        if value != $value {
            raise!("found a malformed or unknown table");
        }
        value
    });
    (@read $name:ident, $this:ident, $tape:ident [$kind:ty] []
     |$this_:tt, $tape_:tt| $body:block) => ({
        #[inline(always)]
        fn read<T: crate::Tape>($this_: &$name, $tape_: &mut T) -> crate::Result<$kind> $body
        read(&$this, $tape)?
    });
}

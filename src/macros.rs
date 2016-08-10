macro_rules! deref {
    (@itemize $($one:item)*) => ($($one)*);
    ($name:ident::$field:tt => $target:ty) => (deref! {
        @itemize

        impl ::std::ops::Deref for $name {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl ::std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! flags {
    ($(#[$attribute:meta])* pub $structure:ident($kind:ident) {
        $($mask:expr => $name:ident,)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $structure(pub $kind);

        impl $structure {
            $(
                #[inline(always)]
                pub fn $name(&self) -> bool {
                    self.0 & $mask > 0
                }
            )*
        }

        impl $crate::Value for $structure {
            #[inline(always)]
            fn read<T: $crate::Tape>(tape: &mut T) -> $crate::Result<Self> {
                let flags = $structure(try!(tape.take::<$kind>()));
                if flags.is_invalid() {
                    raise!("found malformed flags");
                }
                Ok(flags)
            }
        }

        impl From<$structure> for $kind {
            #[inline(always)]
            fn from(flags: $structure) -> $kind {
                flags.0
            }
        }
    );
}

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        table! { @define $(#[$attribute])* pub $structure { $($field ($($kind)+),)* } }
        table! { @implement pub $structure { $($field ($($kind)+) $(|$($argument),+| $body)*,)* } }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)* }
    );
    (@implement pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl $crate::Value for $structure {
            fn read<T: $crate::Tape>(tape: &mut T) -> $crate::Result<Self> {
                let mut table: $structure = unsafe { ::std::mem::uninitialized() };
                $({
                    let value = table!(@read $structure, tape, table, [$($kind)+]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@read $structure:ident, $tape:ident, $table:ident, [$kind:ty]
     |$chair:pat, $band:pat| $body:block) => ({
        #[inline(always)]
        fn read<T: $crate::Tape>($chair: &$structure, $band: &mut T) -> $crate::Result<$kind> $body
        try!(read(&$table, $tape))
    });
    (@read $structure:ident, $tape:ident, $table:expr, [$kind:ty]) => (try!($tape.take()));
}

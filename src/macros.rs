#[doc(hidden)]
#[macro_export]
macro_rules! flags {
    ($(#[$attribute:meta])* pub $name:ident($kind:ident) {
        $($mask:expr => $method:ident,)*
    }) => (
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
                let value = $name(try!(tape.take::<$kind>()));
                if value.is_invalid() {
                    raise!("found malformed flags");
                }
                Ok(value)
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
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
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
        #[derive(Clone, Debug)]
        pub struct $name { $(pub $field: $kind,)* }
    );
    (@implement pub $name:ident {
        $($field:ident ($($kind:tt)+) [$($value:block)*] $(|$($argument:tt),+| $body:block)*,)*
    }) => (
        impl ::Value for $name {
            fn read<T: ::Tape>(tape: &mut T) -> ::Result<Self> {
                let mut table: $name = unsafe { ::std::mem::zeroed() };
                $({
                    let value = table!(@read $name, table, tape [$($kind)+] [$($value)*]
                                       $(|$($argument),+| $body)*);
                    ::std::mem::forget(::std::mem::replace(&mut table.$field, value));
                })*
                Ok(table)
            }
        }
    );
    (@read $name:ident, $this:ident, $tape:ident [$kind:ty] []) => (try!($tape.take()));
    (@read $name:ident, $this:ident, $tape:ident [$kind:ty] [$value:block]) => ({
        let value = try!($tape.take());
        if value != $value {
            raise!("found a malformed or unsupported table");
        }
        value
    });
    (@read $name:ident, $this:ident, $tape:ident [$kind:ty] []
     |$this_:pat, $tape_:pat| $body:block) => ({
        #[inline(always)]
        fn read<T: ::Tape>($this_: &$name, $tape_: &mut T) -> ::Result<$kind> $body
        try!(read(&$this, $tape))
    });
}

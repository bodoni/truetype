//! Compound data types.

#![allow(non_snake_case)]

macro_rules! itemize(($($chunk:item)*) => ($($chunk)*));

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)+
    }) => (
        define_table! { $(#[$attribute])* pub $structure { $($field ($($kind)+),)+ } }
        read_table! { pub $structure { $($field ($($kind)+) $(|$($argument),+| $body)*,)+ } }
    );
}

macro_rules! define_table {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident ($kind:ty),)+ }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)+ }
    });
}

macro_rules! read_table {
    (pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)+
    }) => (
        impl ::tape::Value for $structure {
            fn read<T: ::tape::Tape>(tape: &mut T) -> ::Result<Self> {
                let mut table = $structure::default();
                $(
                    table.$field = read_field!($structure, tape, table, [$($kind)+]
                                               $(|$($argument),+| $body)*);
                )+
                Ok(table)
            }
        }
    );
}

macro_rules! read_field(
    ($structure:ident, $tape:ident, $table:ident,
     [$kind:ty] |$pipe:ident, $chair:ident| $body:block) => ({
        #[inline(always)]
        #[allow(unused_variables)]
        fn read<T: ::tape::Tape>($pipe: &mut T, $chair: &$structure) -> ::Result<$kind> $body
        try!(read($tape, &$table))
    });
    ($structure:ident, $tape:expr, $table:expr, [$kind:ty]) => ({
        try!(::tape::Value::read($tape))
    });
);

macro_rules! read_vector(
    ($tape:ident, $count:expr) => ({
        let count = $count as usize;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(try!(::tape::Value::read($tape)));
        }
        Ok(values)
    });
);

mod char_mapping;
mod offset_table;

pub use self::char_mapping::{CharMapping, CharMappingHeader, CharMappingRecord};
pub use self::char_mapping::{CharMappingEncoding, CharMappingEncoding4, CharMappingEncoding6};
pub use self::offset_table::{OffsetTable, OffsetTableHeader, OffsetTableRecord};

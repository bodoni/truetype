//! Compound data types.

#![allow(non_snake_case)]

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident ($kind:ty),)+ }) => (
        table_define! { $(#[$attribute])* pub $structure { $($field ($kind),)+ } }
        table_read! { pub $structure { $($field,)+ } }
    );
}

macro_rules! table_define {
    ($(#[$attribute:meta])* pub $structure:ident { $($field:ident ($kind:ty),)+ }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)+ }
    });
}

macro_rules! table_read {
    (pub $structure:ident { $($field:ident,)+ }) => (
        impl ::tape::Value for $structure {
            fn read<T: ::tape::Tape>(tape: &mut T) -> ::Result<Self> {
                let mut table = $structure::default();
                $(table.$field = try!(::tape::Value::read(tape));)+
                Ok(table)
            }
        }
    );
}

macro_rules! itemize(($($chunk:item)*) => ($($chunk)*));

mod offset_table;

pub use self::offset_table::{OffsetTable, OffsetTableHeader, OffsetTableRecord};

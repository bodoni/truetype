extern crate truetype;

use truetype::Value;

#[macro_use]
mod common;

use common::setup;

#[test]
fn read() {
    use truetype::WindowsMetrics;

    match ok!(WindowsMetrics::read(&mut setup!(SourceSerif, "OS/2"))) {
        WindowsMetrics::Version3(ref table) => {
            assert!(table.panose == [2, 4, 6, 3, 5, 4, 5, 2, 2, 4]);
            assert!(stringify(&table.vendor_id) == "ADBE");
            assert!(table.break_char == 32);
        }
        _ => unreachable!(),
    }
}

fn stringify<T>(data: &[T]) -> &str {
    use std::{mem, slice, str};

    unsafe {
        let length = data.len() * mem::size_of::<T>();
        let bytes = slice::from_raw_parts(data as *const _ as *const _, length);
        str::from_utf8_unchecked(bytes)
    }
}

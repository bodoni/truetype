extern crate truetype;

use truetype::Value;

#[macro_use]
mod common;

use common::setup;

#[test]
fn read() {
    use truetype::NamingTable;

    match ok!(NamingTable::read(&mut setup!(SourceSerif, "name"))) {
        NamingTable::Format0(ref table) => {
            assert!(table.count == 26);
            assert!(ok!(table.strings())[9] == "Frank Grießhammer");
        }
        _ => unreachable!(),
    }
}

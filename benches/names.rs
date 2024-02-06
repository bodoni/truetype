#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use test::{black_box, Bencher};
use truetype::tables::Names;
use truetype::tape::Read as TapeRead;
use truetype::value::Read as ValueRead;
use truetype::value::Write;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[bench]
fn read(bencher: &mut Bencher) {
    let path = PathBuf::from("tests")
        .join("fixtures")
        .join("OpenSans-Italic.ttf");
    let mut file = ok!(File::open(path));
    bencher.iter(|| {
        ok!(file.jump(195040));
        black_box(ok!(Names::read(&mut file)));
    });
}

#[bench]
fn write(bencher: &mut Bencher) {
    let path = PathBuf::from("tests")
        .join("fixtures")
        .join("OpenSans-Italic.ttf");
    let mut file = ok!(File::open(path));
    ok!(file.jump(195040));
    let table = ok!(Names::read(&mut file));
    bencher.iter(|| {
        let mut cursor = Cursor::new(vec![]);
        black_box(ok!(table.write(&mut cursor)));
    });
}

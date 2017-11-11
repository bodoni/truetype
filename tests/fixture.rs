extern crate truetype;

use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{self, File};
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use truetype::char_mapping::Mapping;

#[derive(Clone, Copy, Debug)]
pub enum Fixture {
    MPlus2P,
    OpenSans,
    SourceSerif,
    VeraMono,
}

impl Fixture {
    pub fn all() -> &'static [Fixture] {
        &[
            Fixture::MPlus2P,
            Fixture::OpenSans,
            Fixture::SourceSerif,
            Fixture::VeraMono,
        ]
    }

    pub fn file_name(&self) -> &'static str {
        match *self {
            Fixture::MPlus2P => "MPlus2P-Regular.ttf",
            Fixture::OpenSans => "OpenSans-Italic.ttf",
            Fixture::SourceSerif => "SourceSerifPro-Regular.otf",
            Fixture::VeraMono => "VeraMono-Roman.ttf",
        }
    }

    pub fn path(&self) -> PathBuf {
        format!("tests/fixtures/{}", self.file_name()).into()
    }

    pub fn offset(&self, table: &str) -> u64 {
        match *self {
            Fixture::MPlus2P => match table {
                "cmap" => 36100,
                _ => unreachable!(),
            },
            Fixture::OpenSans => match table {
                "cmap" => 4276,
                "glyf" => 9608,
                "head" => 316,
                "loca" => 7728,
                "maxp" => 408,
                "post" => 196560,
                _ => unreachable!(),
            },
            Fixture::SourceSerif => match table {
                "OS/2" => 304,
                "cmap" => 15620,
                "head" => 204,
                "hhea" => 260,
                "hmtx" => 55460,
                "maxp" => 296,
                "name" => 400,
                "post" => 17700,
                _ => unreachable!(),
            },
            Fixture::VeraMono => match table {
                "cmap" => 40360,
                _ => unreachable!(),
            },
        }
    }


    fn read_mapping_file<T, B>(buf_read: B) -> HashMap<T, u16>
    where
        T: Eq + Hash + FromStr,
        T::Err: Debug,
        B: BufRead,
    {
        let mut mapping = HashMap::new();
        for line in buf_read.lines().map(|l| l.unwrap()) {
            let mut parts = line.split(" => ");
            mapping.insert(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            );
        }
        mapping
    }

    pub fn mappings(&self) -> Vec<Mapping> {
        let dir_entries =
            fs::read_dir(format!("tests/fixtures/char_mapping/{}", self.file_name())).unwrap();
        let mut mapping_files = Vec::new();
        for entry in dir_entries {
            mapping_files.push(entry.unwrap().file_name().into_string().unwrap());
        }
        mapping_files.sort();

        let mut mappings = Vec::new();
        for mapping_file in mapping_files {
            let mut reader = BufReader::new(
                File::open(format!(
                    "tests/fixtures/char_mapping/{}/{}",
                    self.file_name(),
                    mapping_file,
                )).unwrap(),
            );
            let mut format_line = String::new();
            reader.read_line(&mut format_line).unwrap();
            let format = format_line.trim().parse().unwrap();
            let mapping = match format {
                0 => Mapping::U8(Self::read_mapping_file(reader)),
                4 | 6 => Mapping::U16(Self::read_mapping_file(reader)),
                12 => Mapping::U32(Self::read_mapping_file(reader)),
                14 => Mapping::U32(HashMap::new()),
                _ => unreachable!(),
            };
            mappings.push(mapping);
        }
        mappings
    }
}

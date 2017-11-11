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

    pub fn mappings(&self) -> Vec<Mapping> {
        let path = format!("tests/fixtures/char_mapping/{}", self.file_name());
        let mut file_names = ok!(fs::read_dir(&path))
            .map(|entry| ok!(ok!(entry).file_name().into_string()))
            .collect::<Vec<_>>();
        file_names.sort();
        let mut mappings = Vec::new();
        for file_name in file_names {
            let path = format!("{}/{}", path, file_name);
            let mapping = match read_format(&file_name) {
                0 => Mapping::U8(read_mapping(&path)),
                4 | 6 => Mapping::U16(read_mapping(&path)),
                12 => Mapping::U32(read_mapping(&path)),
                14 => Mapping::U32(HashMap::new()),
                _ => unreachable!(),
            };
            mappings.push(mapping);
        }
        mappings
    }
}

fn read_format(name: &str) -> u8 {
    ok!(
        name.split("format").collect::<Vec<_>>()[1]
            .split(".")
            .collect::<Vec<_>>()[0]
            .parse()
    )
}

fn read_mapping<T>(path: &str) -> HashMap<T, u16>
where
    T: Eq + Hash + FromStr,
    T::Err: Debug,
{
    let reader = BufReader::new(ok!(File::open(path)));
    let mut mapping = HashMap::new();
    for line in reader.lines().map(|line| ok!(line)) {
        let parts = line.split(" => ").collect::<Vec<_>>();
        mapping.insert(ok!(parts[0].parse()), ok!(parts[1].parse()));
    }
    mapping
}

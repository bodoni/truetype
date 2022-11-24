use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

macro_rules! ok(($result:expr) => ($result.unwrap()));

macro_rules! setup(
    ($fixture:ident) => (setup(crate::common::Fixture::$fixture, None));
    ($fixture:ident, $table:expr) => (setup(crate::common::Fixture::$fixture, Some($table)));
);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Fixture {
    MPlus2P,
    OpenSans,
    SourceSerif,
    UbuntuCondensed,
    VeraMono,
}

#[allow(dead_code)]
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
            Fixture::UbuntuCondensed => "UbuntuCondensed-Regular.ttf",
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
            Fixture::UbuntuCondensed => match table {
                "glyf" => 42520,
                "head" => 348,
                "loca" => 39992,
                "maxp" => 440,
                _ => unreachable!(),
            },
            Fixture::VeraMono => match table {
                "cmap" => 40360,
                _ => unreachable!(),
            },
        }
    }

    pub fn mappings(&self) -> Vec<HashMap<u32, u32>> {
        let path = format!("tests/fixtures/char_mappings/{}", self.file_name());
        let mut file_names = ok!(fs::read_dir(&path))
            .map(|entry| ok!(ok!(entry).file_name().into_string()))
            .collect::<Vec<_>>();
        file_names.sort();
        file_names
            .iter()
            .map(|file_name| read_mapping(&format!("{}/{}", path, file_name)))
            .collect::<Vec<_>>()
    }
}

pub fn setup(fixture: Fixture, table: Option<&str>) -> File {
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open(fixture.path()));
    ok!(file.seek(SeekFrom::Start(
        table.map(|table| fixture.offset(table)).unwrap_or(0),
    )));
    file
}

fn read_mapping(path: &str) -> HashMap<u32, u32> {
    let reader = BufReader::new(ok!(File::open(path)));
    let mut mapping = HashMap::new();
    for line in reader.lines().map(|line| ok!(line)) {
        let parts = line.split(" => ").collect::<Vec<_>>();
        mapping.insert(ok!(parts[0].parse()), ok!(parts[1].parse()));
    }
    mapping
}

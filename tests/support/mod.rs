use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

macro_rules! ok(($result:expr) => ($result.unwrap()));

macro_rules! setup(
    ($fixture:ident) => (
        crate::support::setup(crate::support::Fixture::$fixture, None)
    );
    ($fixture:ident, $table:expr) => (
        crate::support::setup(crate::support::Fixture::$fixture, Some($table))
    );
);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Fixture {
    CSSTest,
    MPlus2P,
    OpenSans,
    SourceSerif,
    UbuntuCondensed,
    VeraMono,
    ZenLoop,
}

#[allow(dead_code)]
impl Fixture {
    pub fn file_name(&self) -> &'static str {
        match *self {
            Fixture::CSSTest => "csstest-basic-regular.ttf",
            Fixture::MPlus2P => "MPlus2P-Regular.ttf",
            Fixture::OpenSans => "OpenSans-Italic.ttf",
            Fixture::SourceSerif => "SourceSerifPro-Regular.otf",
            Fixture::UbuntuCondensed => "UbuntuCondensed-Regular.ttf",
            Fixture::VeraMono => "VeraMono-Roman.ttf",
            Fixture::ZenLoop => "ZenLoop-Regular.ttf",
        }
    }

    pub fn path(&self) -> PathBuf {
        PathBuf::from("tests")
            .join("fixtures")
            .join(self.file_name())
    }

    pub fn offset(&self, table: &str) -> u64 {
        match *self {
            Fixture::CSSTest => match table {
                "name" => 101988,
                _ => unreachable!(),
            },
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
                "name" => 195040,
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
            Fixture::ZenLoop => match table {
                "glyf" => 6404,
                "head" => 300,
                "loca" => 5900,
                "maxp" => 392,
                _ => unreachable!(),
            },
        }
    }

    pub fn mappings(&self) -> Vec<HashMap<u32, u32>> {
        let path = PathBuf::from("tests")
            .join("fixtures")
            .join("char_mappings")
            .join(self.file_name());
        let mut file_names = ok!(fs::read_dir(&path))
            .map(|entry| ok!(ok!(entry).file_name().into_string()))
            .collect::<Vec<_>>();
        file_names.sort();
        file_names
            .iter()
            .map(|file_name| read_mapping(&path.join(file_name)))
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

fn read_mapping(path: &Path) -> HashMap<u32, u32> {
    let reader = BufReader::new(ok!(File::open(path)));
    let mut mapping = HashMap::new();
    for line in reader.lines().map(|line| ok!(line)) {
        let parts = line.split(" => ").collect::<Vec<_>>();
        mapping.insert(ok!(parts[0].parse()), ok!(parts[1].parse()));
    }
    mapping
}

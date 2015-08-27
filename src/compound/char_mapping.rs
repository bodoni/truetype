use std::collections::HashMap;

use Result;
use compound::OffsetTableRecord;
use tape::{Tape, Value};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CharMapping {
    pub header: CharMappingHeader,
    pub records: Vec<CharMappingRecord>,
    pub encodings: Vec<CharMappingEncoding>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharMappingEncoding {
    Format4(CharMappingEncoding4),
    Format6(CharMappingEncoding6),
}

table! {
    #[derive(Copy)]
    pub CharMappingHeader {
        version   (u16),
        numTables (u16),
    }
}

table! {
    #[derive(Copy)]
    pub CharMappingRecord {
        platformID (u16),
        encodingID (u16),
        offset     (u32),
    }
}

table! {
    pub CharMappingEncoding4 {
        format        (u16     ),
        length        (u16     ),
        language      (u16     ),
        segCountX2    (u16     ),
        searchRange   (u16     ),
        entrySelector (u16     ),
        rangeShift    (u16     ),
        endCode       (Vec<u16>) |tape, this| { read_vector!(tape, this.segments()) },
        reservedPad   (u16     ),
        startCode     (Vec<u16>) |tape, this| { read_vector!(tape, this.segments()) },
        idDelta       (Vec<i16>) |tape, this| { read_vector!(tape, this.segments()) },
        idRangeOffset (Vec<u16>) |tape, this| { read_vector!(tape, this.segments()) },
        glyphIdArray  (Vec<u16>) |tape, this| { read_vector!(tape, try!(this.array_length())) },
    }
}

table! {
    pub CharMappingEncoding6 {
        format       (u16     ),
        length       (u16     ),
        language     (u16     ),
        firstCode    (u16     ),
        entryCount   (u16     ),
        glyphIdArray (Vec<u16>) |tape, this| { read_vector!(tape, this.entryCount) },
    }
}

impl CharMapping {
    pub fn read<T: Tape>(tape: &mut T, record: &OffsetTableRecord) -> Result<CharMapping> {
        if !try!(record.check(tape, |_, word| word)) {
            raise!("the character-to-glyph mapping is corrupted");
        }
        try!(tape.jump(record.offset as u64));

        let header = match try!(tape.peek::<u16>()) {
            0 => try!(CharMappingHeader::read(tape)),
            _ => raise!("the format of the character-to-glyph mapping header is not supported"),
        };
        let mut records = vec![];
        for _ in 0..header.numTables {
            records.push(try!(CharMappingRecord::read(tape)));
        }
        let mut encodings = vec![];
        for encoding in records.iter() {
            try!(tape.jump(record.offset as u64 + encoding.offset as u64));
            encodings.push(match try!(tape.peek::<u16>()) {
                4 => CharMappingEncoding::Format4(try!(Value::read(tape))),
                6 => CharMappingEncoding::Format6(try!(Value::read(tape))),
                _ => raise!("the format of a character-to-glyph mapping is not supported"),
            });
        }

        Ok(CharMapping { header: header, records: records, encodings: encodings })
    }
}

impl CharMappingEncoding4 {
    pub fn mapping(&self) -> HashMap<u16, u16> {
        let segments = self.segments();

        let mut map = HashMap::new();
        for i in 0..(segments - 1) {
            let startCode = self.startCode[i];
            let idDelta = self.idDelta[i];
            let idRangeOffset = self.idRangeOffset[i];
            for j in startCode..(self.endCode[i] + 1) {
                let index = if idRangeOffset > 0 {
                    let offset = (idRangeOffset / 2 + (j - startCode)) - (segments - i) as u16;
                    self.glyphIdArray[offset as usize]
                } else {
                    (idDelta + j as i16) as u16
                };
                map.insert(j, index);
            }
        }

        map
    }

    fn array_length(&self) -> Result<usize> {
        let segments = self.segments();

        if segments == 0 {
            raise!("a character-to-glyph mapping has no segments");
        }
        if self.startCode[segments - 1] != 0xffff || self.endCode[segments - 1] != 0xffff {
            raise!("a character-to-glyph mapping is malformed");
        }

        let mut length = 0;
        for i in 0..(segments - 1) {
            let startCode = self.startCode[i];
            let idRangeOffset = self.idRangeOffset[i];
            for j in startCode..(self.endCode[i] + 1) {
                if idRangeOffset > 0 {
                    let end = (idRangeOffset / 2 + (j - startCode)) - (segments - i) as u16 + 1;
                    if end > length {
                        length = end;
                    }
                }
            }
        }

        Ok(length as usize)
    }

    #[inline]
    fn segments(&self) -> usize {
        self.segCountX2 as usize / 2
    }
}

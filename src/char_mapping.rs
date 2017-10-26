//! The [char-to-glyph mapping][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/cmap.htm

use std::collections::HashMap;

use {GlyphID, Result, Tape, Value};

/// A char-to-glyph mapping.
#[derive(Clone, Debug)]
pub struct CharMapping {
    pub header: Header,
    pub records: Vec<Record>,
    pub encodings: Vec<Encoding>,
}

/// An encoding of a char-to-glyph mapping.
#[derive(Clone, Debug)]
pub enum Encoding {
    /// Format 4.
    Format4(Encoding4),
    /// Format 6.
    Format6(Encoding6),
}

table! {
    #[doc = "The header of a char-to-glyph mapping."]
    #[derive(Copy)]
    pub Header {
        version     (u16) = { 0 }, // version
        table_count (u16), // numTables
    }
}

table! {
    #[doc = "A record of a char-to-glyph mapping."]
    #[derive(Copy)]
    pub Record {
        platform_id (u16), // platformID
        encoding_id (u16), // encodingID
        offset      (u32), // offset
    }
}

table! {
    #[doc = "A char-to-glyph encoding in format 4."]
    pub Encoding4 {
        format           (u16), // format
        length           (u16), // length
        language         (u16), // language
        segment_count_x2 (u16), // segCountX2
        search_range     (u16), // searchRange
        entry_selector   (u16), // entrySelector
        range_shift      (u16), // rangeShift

        end_codes (Vec<u16>) |this, tape| { // endCode
            tape.take_given(this.segment_count())
        },

        reserved_pad (u16), // reservedPad

        start_codes (Vec<u16>) |this, tape| { // startCode
            tape.take_given(this.segment_count())
        },

        id_deltas (Vec<i16>) |this, tape| { // idDelta
            tape.take_given(this.segment_count())
        },

        id_range_offsets (Vec<u16>) |this, tape| { // idRangeOffset
            tape.take_given(this.segment_count())
        },

        glyph_ids (Vec<GlyphID>) |this, tape| { // glyphIdArray
            tape.take_given(this.glyph_id_count()?)
        },
    }
}

table! {
    #[doc = "A char-to-glyph encoding in format 6."]
    pub Encoding6 {
        format      (u16), // format
        length      (u16), // length
        language    (u16), // language
        first_code  (u16), // firstCode
        entry_count (u16), // entryCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // glyphIdArray
            tape.take_given(this.entry_count as usize)
        },
    }
}

impl Value for CharMapping {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = tape.position()?;
        let header = tape.take::<Header>()?;
        let mut records = vec![];
        for _ in 0..header.table_count {
            records.push(tape.take::<Record>()?);
        }
        let mut encodings = vec![];
        for encoding in records.iter() {
            tape.jump(position + encoding.offset as u64)?;
            encodings.push(match tape.peek::<u16>()? {
                4 => Encoding::Format4(tape.take()?),
                6 => Encoding::Format6(tape.take()?),
                _ => unimplemented!(),
            });
        }
        Ok(CharMapping {
            header: header,
            records: records,
            encodings: encodings,
        })
    }
}

impl Encoding {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u16, u16> {
        match self {
            &Encoding::Format4(ref encoding) => encoding.mapping(),
            &Encoding::Format6(ref encoding) => encoding.mapping(),
        }
    }
}

impl Encoding4 {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u16, u16> {
        use std::num::Wrapping;

        let count = self.segment_count();
        let mut map = HashMap::new();
        for i in 0..(count - 1) {
            let start_code = self.start_codes[i];
            let id_delta = self.id_deltas[i];
            let id_range_offset = self.id_range_offsets[i];
            for j in start_code..(self.end_codes[i] + 1) {
                let id = if id_range_offset > 0 {
                    let offset = (id_range_offset / 2 + (j - start_code)) - (count - i) as u16;
                    self.glyph_ids[offset as usize]
                } else {
                    (Wrapping(id_delta) + Wrapping(j as i16)).0 as u16
                };
                map.insert(j, id);
            }
        }
        map
    }

    fn glyph_id_count(&self) -> Result<usize> {
        macro_rules! reject(() => (raise!("found a malformed char-to-glyph mapping")));
        let count = self.segment_count();
        if count == 0 {
            reject!();
        }
        if self.start_codes[count - 1] != 0xffff || self.end_codes[count - 1] != 0xffff {
            reject!();
        }
        let mut length = 0;
        for i in 0..(count - 1) {
            let start_code = self.start_codes[i];
            let id_range_offset = self.id_range_offsets[i];
            for j in start_code..(self.end_codes[i] + 1) {
                if id_range_offset > 0 {
                    let end = (id_range_offset / 2 + (j - start_code)) - (count - i) as u16 + 1;
                    if end > length {
                        length = end;
                    }
                }
            }
        }
        Ok(length as usize)
    }

    #[inline]
    fn segment_count(&self) -> usize {
        self.segment_count_x2 as usize / 2
    }
}

impl Encoding6 {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u16, u16> {
        unimplemented!();
    }
}

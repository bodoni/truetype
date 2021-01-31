//! The [char-to-glyph mapping][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/cmap.htm

use std::collections::HashMap;

use crate::{GlyphID, Result, Tape, Value};

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
    /// Format 0.
    Format0(Encoding0),
    /// Format 4.
    Format4(Encoding4),
    /// Format 6.
    Format6(Encoding6),
    /// Format 12.
    Format12(Encoding12),
    /// Format 14.
    Format14(Encoding14),
    /// An unknown format.
    Unknown(u16),
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
    #[doc = "A char-to-glyph encoding in format 0."]
    pub Encoding0 {
        format   (u16) = { 0 }, // format
        length   (u16), // length
        language (u16), // language

        glyph_ids (Vec<u8>) |_, tape| { // glyphIdArray
            tape.take_given(256)
        },
    }
}

table! {
    #[doc = "A char-to-glyph encoding in format 4."]
    pub Encoding4 {
        format           (u16) = { 4 }, // format
        length           (u16), // length
        language         (u16), // language
        segment_count_x2 (u16), // segCountX2
        search_range     (u16), // searchRange
        entry_selector   (u16), // entrySelector
        range_shift      (u16), // rangeShift

        end_codes (Vec<u16>) |this, tape| { // endCode
            tape.take_given(this.segment_count())
        },

        reserved (u16) = { 0 }, // reservedPad

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
        format      (u16) = { 6 }, // format
        length      (u16), // length
        language    (u16), // language
        first_code  (u16), // firstCode
        entry_count (u16), // entryCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // glyphIdArray
            tape.take_given(this.entry_count as usize)
        },
    }
}

table! {
    #[doc = "A char-to-glyph encoding in format 12."]
    pub Encoding12 {
        format      (u16) = { 12 }, // format
        reserved    (u16) = { 0 }, // reserved
        length      (u32), // length
        language    (u32), // language
        group_count (u32), // numGroups

        groups (Vec<SequentialGroup>) |this, tape| { // groups
            tape.take_given(this.group_count as usize)
        },
    }
}

table! {
    #[doc = "A char-to-glyph encoding in format 14."]
    pub Encoding14 {
        format         (u16) = { 14 }, // format
        length         (u32), // length
        selector_count (u32), // numVarSelectorRecords

        selectors (Vec<VariationSelector>) |this, tape| { // varSelector
            tape.take_given(this.selector_count as usize)
        },
    }
}

table! {
    #[doc = "A sequential mapping group."]
    pub SequentialGroup {
        start_code     (u32), // startCharCode
        end_code       (u32), // endCharCode
        start_glyph_id (u32), // startGlyphID
    }
}

table! {
    #[doc = "A variation selector."]
    pub VariationSelector {
        character (u32) |_, tape| { // varSelector
            let buffer: [u8; 3] = tape.take()?;
            Ok(u32::from_be(
                u32::from(buffer[0]) << 8 |
                u32::from(buffer[1]) << 16 |
                u32::from(buffer[2]) << 24
            ))
        },

        default_uvs_offset     (u32), // defaultUVSOffset
        non_default_uvs_offset (u32), // nonDefaultUVSOffset
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
                0 => Encoding::Format0(tape.take()?),
                4 => Encoding::Format4(tape.take()?),
                6 => Encoding::Format6(tape.take()?),
                12 => Encoding::Format12(tape.take()?),
                14 => Encoding::Format14(tape.take()?),
                format => Encoding::Unknown(format),
            });
        }
        Ok(CharMapping {
            header: header,
            records: records,
            encodings: encodings,
        })
    }
}

impl Encoding0 {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u8, GlyphID> {
        let mut mapping = HashMap::new();
        for (i, glyph_id) in self.glyph_ids.iter().enumerate() {
            mapping.insert(i as u8, *glyph_id as GlyphID);
        }
        mapping
    }
}

impl Encoding4 {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u16, GlyphID> {
        use std::num::Wrapping;

        let count = self.segment_count();
        let mut mapping = HashMap::new();
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
                mapping.insert(j, id);
            }
        }
        mapping
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
    pub fn mapping(&self) -> HashMap<u16, GlyphID> {
        let mut mapping = HashMap::new();
        for (i, glyph_id) in self.glyph_ids.iter().enumerate() {
            mapping.insert(self.first_code + i as u16, *glyph_id);
        }
        mapping
    }
}

impl Encoding12 {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u32, GlyphID> {
        let mut mapping = HashMap::new();
        for group in &self.groups {
            for i in 0..(group.end_code - group.start_code + 1) {
                mapping.insert(
                    group.start_code + i,
                    group.start_glyph_id as u16 + i as u16,
                );
            }
        }
        mapping
    }
}

impl Encoding14 {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u32, GlyphID> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::Tape;
    use super::VariationSelector;

    #[test]
    fn variation_selector_record() {
        let mut buffer = Cursor::new(vec![
            0x02u8,
            0x01,
            0xFF,
            0x00,
            0x02,
            0x01,
            0xFF,
            0xAA,
            0x02,
            0x01,
            0xFF,
        ]);
        let record = buffer.take::<VariationSelector>().unwrap();
        assert!(record.character == 0x000201FF);
        assert!(record.default_uvs_offset == 0x000201FF);
        assert!(record.non_default_uvs_offset == 0xAA0201FF);
    }
}

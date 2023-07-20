//! The [character-to-glyph mapping][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/cmap

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use crate::{GlyphID, Result, Tape, Value};

/// A character-to-glyph mapping.
#[derive(Clone, Debug)]
pub struct CharacterMapping {
    pub header: Header,
    pub records: Vec<Record>,
    pub encodings: Vec<Encoding>,
}

/// An encoding of a character-to-glyph mapping.
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
    #[doc = "The header of a character-to-glyph mapping."]
    #[derive(Copy)]
    pub Header {
        version     (u16) = { 0 }, // version
        table_count (u16), // numTables
    }
}

table! {
    #[doc = "A record of a character-to-glyph mapping."]
    #[derive(Copy)]
    pub Record {
        platform_id (u16), // platformID
        encoding_id (u16), // encodingID
        offset      (u32), // offset
    }
}

table! {
    #[doc = "A character-to-glyph encoding in format 0."]
    pub Encoding0 {
        format   (u16) = { 0 }, // format
        size     (u16), // length
        language (u16), // language

        glyph_ids (Vec<u8>) |_, tape| { // glyphIdArray
            tape.take_given(256)
        },
    }
}

table! {
    #[doc = "A character-to-glyph encoding in format 4."]
    pub Encoding4 {
        format           (u16) = { 4 }, // format
        size             (u16), // length
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
    #[doc = "A character-to-glyph encoding in format 6."]
    pub Encoding6 {
        format      (u16) = { 6 }, // format
        size        (u16), // length
        language    (u16), // language
        first_code  (u16), // firstCode
        entry_count (u16), // entryCount

        glyph_ids (Vec<GlyphID>) |this, tape| { // glyphIdArray
            tape.take_given(this.entry_count as usize)
        },
    }
}

table! {
    #[doc = "A character-to-glyph encoding in format 12."]
    pub Encoding12 {
        format      (u16) = { 12 }, // format
        reserved    (u16) = { 0 }, // reserved
        size        (u32), // length
        language    (u32), // language
        group_count (u32), // numGroups

        groups (Vec<SequentialGroup>) |this, tape| { // groups
            tape.take_given(this.group_count as usize)
        },
    }
}

table! {
    #[doc = "A character-to-glyph encoding in format 14."]
    pub Encoding14 {
        format         (u16) = { 14 }, // format
        size           (u32), // length
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

impl Value for CharacterMapping {
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
        Ok(CharacterMapping {
            header,
            records,
            encodings,
        })
    }
}

impl Encoding0 {
    /// Return the characters.
    #[inline]
    pub fn characters<T: From<u8>>(&self) -> Vec<(T, T)> {
        vec![(T::from(0), T::from(255))]
    }

    /// Return the mapping.
    pub fn mapping<T: From<u8> + Eq + Hash>(&self) -> HashMap<T, GlyphID> {
        let mut result = HashMap::new();
        for (i, glyph_id) in self.glyph_ids.iter().enumerate() {
            result.insert(T::from(i as u8), *glyph_id as GlyphID);
        }
        result
    }
}

impl Encoding4 {
    /// Return the characters.
    pub fn characters<T: From<u16>>(&self) -> Vec<(T, T)> {
        let segment_count = self.segment_count();
        if segment_count == 0 {
            return Default::default();
        }
        (0..(segment_count - 1))
            .map(|i| (T::from(self.start_codes[i]), T::from(self.end_codes[i])))
            .collect()
    }

    /// Return the mapping.
    pub fn mapping<T: From<u16> + Eq + Hash>(&self) -> HashMap<T, GlyphID> {
        use std::num::Wrapping;

        let segment_count = self.segment_count();
        if segment_count == 0 {
            return Default::default();
        }
        let mut result = HashMap::new();
        for i in 0..(segment_count - 1) {
            let start_code = self.start_codes[i];
            let id_delta = self.id_deltas[i];
            let id_range_offset = self.id_range_offsets[i];
            for j in start_code..(self.end_codes[i] + 1) {
                let id = if id_range_offset > 0 {
                    let offset =
                        (id_range_offset / 2 + (j - start_code)) - (segment_count - i) as u16;
                    self.glyph_ids[offset as usize]
                } else {
                    (Wrapping(id_delta) + Wrapping(j as i16)).0 as u16
                };
                result.insert(T::from(j), id);
            }
        }
        result
    }

    fn glyph_id_count(&self) -> Result<usize> {
        macro_rules! reject(() => (raise!("found a malformed character-to-glyph mapping")));
        let segment_count = self.segment_count();
        if segment_count == 0 {
            return Ok(0);
        }
        if self.start_codes[segment_count - 1] != 0xffff
            || self.end_codes[segment_count - 1] != 0xffff
        {
            reject!();
        }
        let mut count = 0;
        for i in 0..(segment_count - 1) {
            let start_code = self.start_codes[i];
            let id_range_offset = self.id_range_offsets[i];
            for j in start_code..(self.end_codes[i] + 1) {
                if id_range_offset > 0 {
                    let end =
                        (id_range_offset / 2 + (j - start_code)) - (segment_count - i) as u16 + 1;
                    if end > count {
                        count = end;
                    }
                }
            }
        }
        Ok(count as usize)
    }

    #[inline]
    fn segment_count(&self) -> usize {
        self.segment_count_x2 as usize / 2
    }
}

impl Encoding6 {
    /// Return the characters.
    pub fn characters<T: From<u16>>(&self) -> Vec<(T, T)> {
        if self.entry_count == 0 {
            return Default::default();
        }
        vec![(
            T::from(self.first_code),
            T::from(self.first_code + self.entry_count - 1),
        )]
    }

    /// Return the mapping.
    pub fn mapping<T: From<u16> + Eq + Hash>(&self) -> HashMap<T, GlyphID> {
        let mut result = HashMap::new();
        for (i, glyph_id) in self.glyph_ids.iter().enumerate() {
            result.insert(T::from(self.first_code + i as u16), *glyph_id);
        }
        result
    }
}

impl Encoding12 {
    /// Return the characters.
    pub fn characters<T: From<u32>>(&self) -> Vec<(T, T)> {
        Default::default()
    }

    /// Return the mapping.
    pub fn mapping<T: From<u32> + Eq + Hash>(&self) -> HashMap<T, GlyphID> {
        let mut result = HashMap::new();
        for group in &self.groups {
            for i in 0..(group.end_code - group.start_code + 1) {
                result.insert(
                    T::from(group.start_code + i),
                    group.start_glyph_id as u16 + i as u16,
                );
            }
        }
        result
    }
}

impl Encoding14 {
    /// Return the characters.
    ///
    /// It is not implemented yet.
    #[inline]
    pub fn characters<T: From<u32>>(&self) -> Vec<(T, T)> {
        Default::default()
    }

    /// Return the mapping.
    ///
    /// It is not implemented yet.
    #[inline]
    pub fn mapping<T: From<u32> + Eq + Hash>(&self) -> HashMap<T, GlyphID> {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::VariationSelector;
    use crate::Tape;

    #[test]
    fn variation_selector_record() {
        let mut tape = Cursor::new(vec![
            0x02u8, 0x01, 0xFF, 0x00, 0x02, 0x01, 0xFF, 0xAA, 0x02, 0x01, 0xFF,
        ]);
        let record = tape.take::<VariationSelector>().unwrap();
        assert!(record.character == 0x000201FF);
        assert!(record.default_uvs_offset == 0x000201FF);
        assert!(record.non_default_uvs_offset == 0xAA0201FF);
    }
}

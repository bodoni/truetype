//! The [horizontal metrics][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/hmtx

use crate::tables::horizontal_header::HorizontalHeader;
use crate::tables::maximum_profile::MaximumProfile;
use crate::{GlyphID, Result};

table! {
    @define
    /// Horizontal metrics.
    pub HorizontalMetrics {
        records            (Vec<Record>), // hMetrics
        left_side_bearings (Vec<i16>   ), // leftSideBearing
    }
}

table! {
    /// A record of horizontal metrics.
    #[derive(Copy)]
    pub Record { // longHorMetric
        advance_width     (u16), // advanceWidth
        left_side_bearing (i16), // lsb
    }
}

impl HorizontalMetrics {
    /// Return the advance width and left side bearing.
    pub fn get(&self, glyph_id: GlyphID) -> (u16, i16) {
        let mut index = glyph_id as usize;
        let longs = self.records.len();
        if index < longs {
            (
                self.records[index].advance_width,
                self.records[index].left_side_bearing,
            )
        } else {
            let shorts = self.left_side_bearings.len();
            index -= longs;
            if index < shorts {
                (
                    self.records[longs - 1].advance_width,
                    self.left_side_bearings[index],
                )
            } else {
                (
                    self.records[longs - 1].advance_width,
                    self.left_side_bearings[shorts - 1],
                )
            }
        }
    }
}

impl<'l> crate::walue::Read<'l> for HorizontalMetrics {
    type Parameter = (&'l HorizontalHeader, &'l MaximumProfile);

    fn read<T: crate::tape::Read>(
        tape: &mut T,
        (header, profile): Self::Parameter,
    ) -> Result<Self> {
        let metric_count = header.horizontal_metric_count as usize;
        let glyph_count = profile.glyph_count();
        if metric_count == 0 || metric_count > glyph_count {
            raise!("found a malformed horizontal header");
        }
        let bearing_count = glyph_count - metric_count;
        let mut table = HorizontalMetrics {
            records: Vec::with_capacity(metric_count),
            left_side_bearings: Vec::with_capacity(bearing_count),
        };
        for _ in 0..metric_count {
            table.records.push(tape.take()?);
        }
        for _ in 0..bearing_count {
            table.left_side_bearings.push(tape.take()?);
        }
        Ok(table)
    }
}

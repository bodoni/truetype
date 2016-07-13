//! The horizontal metrics.

use {HorizontalHeader, MaximumProfile, Result, Tape, Value};

table! {
    @define
    #[doc = "Horizontal metrics."]
    pub HorizontalMetrics {
        records            (Vec<Record>), // hMetrics
        left_side_bearings (Vec<i16>   ), // leftSideBearing
    }
}

table! {
    #[doc = "A record of horizontal metrics."]
    #[derive(Copy)]
    pub Record { // longHorMetric
        advance_width     (u16), // advanceWidth
        left_side_bearing (i16), // lsb
    }
}

impl HorizontalMetrics {
    /// Read the table.
    pub fn read<T: Tape>(tape: &mut T, header: &HorizontalHeader, profile: &MaximumProfile)
                         -> Result<Self> {

        let metrics = header.horizontal_metric_count as usize;
        let glyphs = profile.glyph_count();
        debug_assert!(metrics <= glyphs);
        let bearings = glyphs - metrics;
        let mut table = HorizontalMetrics {
            records: Vec::with_capacity(metrics),
            left_side_bearings: Vec::with_capacity(bearings),
        };
        for _ in 0..metrics {
            table.records.push(try!(Value::read(tape)));
        }
        for _ in 0..bearings {
            table.left_side_bearings.push(try!(Value::read(tape)));
        }
        Ok(table)
    }
}

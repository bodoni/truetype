//! The horizontal metrics.

use {HorizontalHeader, MaximumProfile, Result, Tape, Walue};

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

impl<'l> Walue<(&'l HorizontalHeader, &'l MaximumProfile)> for HorizontalMetrics {
    fn read<T: Tape>(tape: &mut T, (header, profile): (&HorizontalHeader, &MaximumProfile))
                     -> Result<Self> {

        let metric_count = header.horizontal_metric_count as usize;
        let glyph_count = profile.glyph_count();
        if metric_count > glyph_count {
            raise!("found a malformed horizontal header");
        }
        let bearing_count = glyph_count - metric_count;
        let mut table = HorizontalMetrics {
            records: Vec::with_capacity(metric_count),
            left_side_bearings: Vec::with_capacity(bearing_count),
        };
        for _ in 0..metric_count {
            table.records.push(read_value!(tape));
        }
        for _ in 0..bearing_count {
            table.left_side_bearings.push(read_value!(tape));
        }
        Ok(table)
    }
}

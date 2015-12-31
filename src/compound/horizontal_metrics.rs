use Result;
use compound::{HorizontalHeader, MaximumProfile};
use tape::{Tape, Value};

define_table! {
    #[doc = "Horizontal metrics."]
    pub HorizontalMetrics {
        hMetrics        (Vec<LongHorizontalMetric>),
        leftSideBearing (Vec<i16>                 ),
    }
}

table! {
    #[doc = "A record of horizontal metrics."]
    #[derive(Copy)]
    pub LongHorizontalMetric {
        advanceWidth (u16),
        lsb          (i16),
    }
}

impl HorizontalMetrics {
    /// Read the table.
    pub fn read<T: Tape>(tape: &mut T, header: &HorizontalHeader, profile: &MaximumProfile)
                         -> Result<Self> {

        let metrics = header.horizontal_metric_count as usize;
        let glyphs = profile.glyphs();
        debug_assert!(metrics <= glyphs);
        let bearings = glyphs - metrics;
        let mut table = HorizontalMetrics {
            hMetrics: Vec::with_capacity(metrics),
            leftSideBearing: Vec::with_capacity(bearings),
        };
        for _ in 0..metrics {
            table.hMetrics.push(try!(Value::read(tape)));
        }
        for _ in 0..bearings {
            table.leftSideBearing.push(try!(Value::read(tape)));
        }
        Ok(table)
    }
}

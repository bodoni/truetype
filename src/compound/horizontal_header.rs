use primitive::Fixed;
use tape::Value;

macro_rules! read_version(
    ($tape:ident) => ({
        let value = try!(Value::read($tape));
        if value != Fixed(0x00010000) {
            raise!("the version of the horizontal header is not supported");
        }
        Ok(value)
    });
);

table! {
    #[doc = "A horizontal header."]
    #[derive(Copy)]
    pub HorizontalHeader {
        version                 (Fixed) |tape, this| { read_version!(tape) },
        ascender                (i16  ), // Ascender
        descender               (i16  ), // Descender
        line_gap                (i16  ), // LineGap
        advance_width_max       (u16  ), // advanceWidthMax
        left_side_bearing_min   (i16  ), // minLeftSideBearing
        right_side_bearing_min  (i16  ), // minRightSideBearing
        x_extent_max            (i16  ), // xMaxExtent
        caret_slope_rise        (i16  ), // caretSlopeRise
        caret_slope_run         (i16  ), // caretSlopeRun
        caret_offset            (i16  ), // caretOffset
        reserved1               (i16  ),
        reserved2               (i16  ),
        reserved3               (i16  ),
        reserved4               (i16  ),
        metric_data_format      (i16  ), // metricDataFormat
        horizontal_metric_count (u16  ), // numberOfHMetrics
    }
}

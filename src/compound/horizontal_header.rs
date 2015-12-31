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
        version             (Fixed) |tape, this| { read_version!(tape) },
        ascender            (i16  ),
        descender           (i16  ),
        lineGap             (i16  ),
        advanceWidthMax     (u16  ),
        minLeftSideBearing  (i16  ),
        minRightSideBearing (i16  ),
        xMaxExtent          (i16  ),
        caretSlopeRise      (i16  ),
        caretSlopeRun       (i16  ),
        caretOffset         (i16  ),
        reserved1           (i16  ),
        reserved2           (i16  ),
        reserved3           (i16  ),
        reserved4           (i16  ),
        metricDataFormat    (i16  ),
        numberOfHMetrics    (u16  ),
    }
}

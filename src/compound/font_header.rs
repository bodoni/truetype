use primitive::Fixed;

table! {
    #[derive(Copy)]
    pub FontHeader {
        version            (Fixed),
        fontRevision       (Fixed),
        checkSumAdjustment (u32  ),
        magicNumber        (u32  ),
        flags              (u16  ),
        unitsPerEm         (u16  ),
        created            (i64  ),
        modified           (i64  ),
        xMin               (i16  ),
        yMin               (i16  ),
        xMax               (i16  ),
        yMax               (i16  ),
        macStyle           (u16  ),
        lowestRecPPEM      (u16  ),
        fontDirectionHint  (i16  ),
        indexToLocFormat   (i16  ),
        glyphDataFormat    (i16  ),
    }
}

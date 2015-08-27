use primitive::Fixed;

use Result;
use tape::{Tape, Value};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaximumProfile {
    Version05(MaximumProfile05),
    Version10(MaximumProfile10),
}

table! {
    #[derive(Copy)]
    pub MaximumProfile05 {
        version   (Fixed),
        numGlyphs (u16  ),
    }
}

table! {
    #[derive(Copy)]
    pub MaximumProfile10 {
        version               (Fixed),
        numGlyphs             (u16  ),
        maxPoints             (u16  ),
        maxContours           (u16  ),
        maxCompositePoints    (u16  ),
        maxCompositeContours  (u16  ),
        maxZones              (u16  ),
        maxTwilightPoints     (u16  ),
        maxStorage            (u16  ),
        maxFunctionDefs       (u16  ),
        maxInstructionDefs    (u16  ),
        maxStackElements      (u16  ),
        maxSizeOfInstructions (u16  ),
        maxComponentElements  (u16  ),
        maxComponentDepth     (u16  ),
    }
}

impl MaximumProfile {
    pub fn glyphs(&self) -> usize {
        match self {
            &MaximumProfile::Version05(ref profile) => profile.numGlyphs as usize,
            &MaximumProfile::Version10(ref profile) => profile.numGlyphs as usize,
        }
    }
}

impl Value for MaximumProfile {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<Fixed>()) {
            Fixed(0x00005000) => MaximumProfile::Version05(try!(Value::read(tape))),
            Fixed(0x00010000) => MaximumProfile::Version10(try!(Value::read(tape))),
            _ => raise!("the format of the maximum profile is not supported"),
        })
    }
}

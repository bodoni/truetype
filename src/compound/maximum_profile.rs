use primitive::Fixed;

use Result;
use tape::{Tape, Value};

/// A maximum profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaximumProfile {
    /// Version 0.5.
    Version05(MaximumProfile05),
    /// Version 1.0.
    Version10(MaximumProfile10),
}

table! {
    #[doc = "A maximum profile of version 0.5."]
    #[derive(Copy)]
    pub MaximumProfile05 {
        version     (Fixed),
        glyph_count (u16  ), // numGlyphs
    }
}

table! {
    #[doc = "A maximum profile of version 1.0."]
    #[derive(Copy)]
    pub MaximumProfile10 {
        version                     (Fixed),
        glyph_count                 (u16  ), // numGlyphs
        max_points                  (u16  ), // maxPoints
        max_contours                (u16  ), // maxContours
        max_composite_points        (u16  ), // maxCompositePoints
        max_composite_contours      (u16  ), // maxCompositeContours
        max_zones                   (u16  ), // maxZones
        max_twilight_points         (u16  ), // maxTwilightPoints
        max_storage                 (u16  ), // maxStorage
        max_function_definitions    (u16  ), // maxFunctionDefs
        max_instruction_definitions (u16  ), // maxInstructionDefs
        max_stack_elements          (u16  ), // maxStackElements
        max_size_of_instructions    (u16  ), // maxSizeOfInstructions
        max_component_elements      (u16  ), // maxComponentElements
        max_component_depth         (u16  ), // maxComponentDepth
    }
}

impl MaximumProfile {
    /// Return the number of glyphs.
    pub fn glyph_count(&self) -> usize {
        match self {
            &MaximumProfile::Version05(ref profile) => profile.glyph_count as usize,
            &MaximumProfile::Version10(ref profile) => profile.glyph_count as usize,
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

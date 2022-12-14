//! The [maximum profile][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/maxp

use crate::number::q32;
use crate::{Result, Tape, Value};

/// A maximum profile.
#[derive(Clone, Debug)]
pub enum MaximumProfile {
    /// Version 0.5.
    Version0(MaximumProfile0),
    /// Version 1.
    Version1(MaximumProfile1),
}

table! {
    #[doc = "A maximum profile of version 0.5."]
    #[derive(Copy)]
    pub MaximumProfile0 {
        version     (q32), // version
        glyph_count (u16), // numGlyphs
    }
}

table! {
    #[doc = "A maximum profile of version 1."]
    #[derive(Copy)]
    pub MaximumProfile1 {
        version                     (q32), // version
        glyph_count                 (u16), // numGlyphs
        max_points                  (u16), // maxPoints
        max_contours                (u16), // maxContours
        max_composite_points        (u16), // maxCompositePoints
        max_composite_contours      (u16), // maxCompositeContours
        max_zones                   (u16), // maxZones
        max_twilight_points         (u16), // maxTwilightPoints
        max_storage                 (u16), // maxStorage
        max_function_definitions    (u16), // maxFunctionDefs
        max_instruction_definitions (u16), // maxInstructionDefs
        max_stack_elements          (u16), // maxStackElements
        max_size_of_instructions    (u16), // maxSizeOfInstructions
        max_component_elements      (u16), // maxComponentElements
        max_component_depth         (u16), // maxComponentDepth
    }
}

impl MaximumProfile {
    /// Return the number of glyphs.
    pub fn glyph_count(&self) -> usize {
        match self {
            &MaximumProfile::Version0(ref profile) => profile.glyph_count as usize,
            &MaximumProfile::Version1(ref profile) => profile.glyph_count as usize,
        }
    }
}

impl Value for MaximumProfile {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<q32>()? {
            q32(0x00005000) => MaximumProfile::Version0(tape.take()?),
            q32(0x00010000) => MaximumProfile::Version1(tape.take()?),
            _ => raise!("found an unknown version of the maximum profile"),
        })
    }
}

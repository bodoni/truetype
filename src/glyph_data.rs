//! The [glyph data][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/glyf

use crate::{q16, GlyphMapping, Result, Tape, Walue};

/// Glyph data.
#[derive(Clone, Debug)]
pub struct GlyphData(pub Vec<Option<Glyph>>);

table! {
    #[doc = "A glyph."]
    pub Glyph {
        contour_count (i16), // numberOfContours
        min_x         (i16), // xMin
        min_y         (i16), // yMin
        max_x         (i16), // xMax
        max_y         (i16), // yMax

        description (Description) |this, tape| {
            tape.take_given(this.contour_count)
        },
    }
}

/// A glyph description.
#[derive(Clone, Debug)]
pub enum Description {
    /// A simple-glyph description.
    Simple(SimpleDescription),
    /// A composite-glyph description.
    Composite(CompositeDescription),
}

table! {
    @define
    #[doc = "A simple-glyph description."]
    pub SimpleDescription {
        end_points       (Vec<u16>       ), // endPtsOfContours
        instruction_size (u16            ), // instructionLength
        instructions     (Vec<u8>        ), // instructions
        flags            (Vec<PointFlags>), // flags
        x                (Vec<i16>       ), // xCoordinates
        y                (Vec<i16>       ), // yCoordinates
    }
}

/// A composite-glyph description.
#[derive(Clone, Debug)]
pub struct CompositeDescription {
    pub components: Vec<Component>,
    pub instruction_size: u16,
    pub instructions: Vec<u8>,
}

table! {
    #[doc = "A component of a composite glyph."]
    #[derive(Copy)]
    pub Component {
        flags       (ComponentFlags), // flags
        glyph_index (u16           ), // glyphIndex

        arguments (Arguments) |this, tape| { // argument1, argument2
            tape.take_given(this.flags)
        },

        options (Options) |this, tape| {
            tape.take_given(this.flags)
        },
    }
}

flags! {
    #[doc = "Point flags."]
    pub PointFlags(u8) {
        0b0000_0001 => is_on_curve,
        0b0000_0010 => is_x_short,
        0b0000_0100 => is_y_short,
        0b0000_1000 => is_repeated,
        0b0001_0000 => is_x_positive,
        0b0001_0000 => is_x_same,
        0b0010_0000 => is_y_positive,
        0b0010_0000 => is_y_same,
        0b0100_0000 => is_overlap_simple,
        0b1000_0000 => is_invalid,
    }
}

flags! {
    #[doc = "Component flags."]
    pub ComponentFlags(u16) {
        0b0000_0000_0000_0001 => are_arguments_words,
        0b0000_0000_0000_0010 => are_arguments_xy,
        0b0000_0000_0000_0100 => should_round_xy_to_grid,
        0b0000_0000_0000_1000 => has_scalar_scale,
        0b0000_0000_0010_0000 => has_more_components,
        0b0000_0000_0100_0000 => has_vector_scale,
        0b0000_0000_1000_0000 => has_matrix_scale,
        0b0000_0001_0000_0000 => has_instructions,
        0b0000_0010_0000_0000 => should_use_metrics,
        0b0000_0100_0000_0000 => has_overlap,
        0b0000_1000_0000_0000 => is_offset_scaled,
        0b0001_0000_0000_0000 => is_offset_unscaled,
        0b1110_0000_0001_0000 => is_invalid,
    }
}

/// Arguments of a component.
#[derive(Clone, Copy, Debug)]
pub enum Arguments {
    /// Offsets relative to the current point.
    Offsets(i16, i16),
    /// Indices of the points to match.
    Indices(u16, u16),
}

/// Options of a component.
#[derive(Clone, Copy, Debug)]
pub enum Options {
    /// No options.
    None,
    /// A scaling coefficient for both coordinates.
    Scalar(q16),
    /// Separate scaling coefficients for the two coordinates.
    Vector(q16, q16),
    /// A 2-by-2 affine transformation matrix.
    Matrix(q16, q16, q16, q16),
}

deref! { GlyphData::0 => [Option<Glyph>] }

impl<'l> Walue<'l> for GlyphData {
    type Parameter = &'l GlyphMapping;

    fn read<T: Tape>(tape: &mut T, mapping: &GlyphMapping) -> Result<Self> {
        macro_rules! reject(
            () => (
                raise!("found a malformed glyph-to-location mapping")
            );
            ($index:ident) => (
                raise!("found a malformed glyph-to-location mapping at index {}", $index)
            );
            ($index:ident, $error:ident) => (
                raise!(@from $error, "found a malformed glyph-to-location mapping at index {}", $index)
            );
        );
        let offsets: Vec<_> = match mapping {
            &GlyphMapping::HalfOffsets(ref offsets) => {
                offsets.iter().map(|&offset| 2 * (offset as u64)).collect()
            }
            &GlyphMapping::Offsets(ref offsets) => {
                offsets.iter().map(|&offset| offset as u64).collect()
            }
        };
        if offsets.is_empty() {
            reject!();
        }
        let glyph_count = offsets.len() - 1;
        let mut glyphs = Vec::with_capacity(glyph_count);
        let position = tape.position()?;
        for i in 0..glyph_count {
            if offsets[i] > offsets[i + 1] {
                reject!(i);
            }
            if offsets[i] == offsets[i + 1] {
                glyphs.push(None);
                continue;
            }
            tape.jump(position + offsets[i])?;
            match tape.take() {
                Ok(glyph) => glyphs.push(Some(glyph)),
                Err(error) => reject!(i, error),
            }
            if tape.position()? > position + offsets[i + 1] {
                reject!(i);
            }
        }
        Ok(GlyphData(glyphs))
    }
}

impl Default for Description {
    #[inline]
    fn default() -> Self {
        Description::Simple(SimpleDescription::default())
    }
}

impl Walue<'static> for Description {
    type Parameter = i16;

    fn read<T: Tape>(tape: &mut T, contour_count: i16) -> Result<Self> {
        if contour_count < -1 {
            raise!("found a malformed glyph");
        }
        if contour_count >= 0 {
            return Ok(Description::Simple(
                tape.take_given(contour_count as usize)?,
            ));
        }
        let mut components = vec![];
        let mut component_count = 0;
        let mut has_more_components = true;
        let mut has_instructions = false;
        while has_more_components {
            components.push(tape.take::<Component>()?);
            has_instructions |= components[component_count].flags.has_instructions();
            has_more_components = components[component_count].flags.has_more_components();
            component_count += 1;
        }
        let instruction_size = if has_instructions {
            tape.take::<u16>()?
        } else {
            0
        };
        let instructions = tape.take_bytes(instruction_size as usize)?;
        Ok(Description::Composite(CompositeDescription {
            components: components,
            instruction_size: instruction_size,
            instructions: instructions,
        }))
    }
}

impl Walue<'static> for SimpleDescription {
    type Parameter = usize;

    fn read<T: Tape>(tape: &mut T, contour_count: usize) -> Result<Self> {
        macro_rules! reject(() => (raise!("found a malformed glyph description")));

        let end_points = tape.take_given::<Vec<u16>>(contour_count)?;
        for i in 1..contour_count {
            if end_points[i - 1] > end_points[i] {
                reject!();
            }
        }
        let point_count = end_points.last().map(|&i| i as usize + 1).unwrap_or(0);

        let instruction_size = tape.take()?;
        let instructions = tape.take_bytes(instruction_size as usize)?;

        let mut flags = Vec::with_capacity(point_count);
        let mut flag_count = 0;
        while flag_count < point_count {
            let flag = tape.take::<PointFlags>()?;
            if flag.is_invalid() {
                reject!();
            }
            let count = 1 + if flag.is_repeated() {
                tape.take::<u8>()? as usize
            } else {
                0
            };
            if flag_count + count > point_count {
                reject!();
            }
            for _ in 0..count {
                flags.push(flag);
            }
            flag_count += count;
        }

        macro_rules! read_coordinates(
            ($is_short:ident, $is_positive:ident, $is_same:ident) => ({
                let mut values = Vec::with_capacity(point_count);
                for i in 0..point_count {
                    let value = if flags[i].$is_short() {
                        let value = tape.take::<u8>()? as i16;
                        if flags[i].$is_positive() { value } else { -value }
                    } else {
                        if flags[i].$is_same() { 0 } else { tape.take::<i16>()? }
                    };
                    values.push(value);
                }
                values
            });
        );
        let x = read_coordinates!(is_x_short, is_x_positive, is_x_same);
        let y = read_coordinates!(is_y_short, is_y_positive, is_y_same);

        Ok(SimpleDescription {
            end_points: end_points,
            instruction_size: instruction_size,
            instructions: instructions,
            flags: flags,
            x: x,
            y: y,
        })
    }
}

impl Default for Arguments {
    #[inline]
    fn default() -> Self {
        Arguments::Offsets(0, 0)
    }
}

impl Walue<'static> for Arguments {
    type Parameter = ComponentFlags;

    fn read<T: Tape>(tape: &mut T, flags: ComponentFlags) -> Result<Self> {
        match (flags.are_arguments_words(), flags.are_arguments_xy()) {
            (true, true) => {
                let x = tape.take::<i16>()?;
                let y = tape.take::<i16>()?;
                Ok(Arguments::Offsets(x, y))
            }
            (false, true) => {
                let x = tape.take::<i8>()? as i16;
                let y = tape.take::<i8>()? as i16;
                Ok(Arguments::Offsets(x, y))
            }
            (true, false) => {
                let i = tape.take::<u16>()?;
                let j = tape.take::<u16>()?;
                Ok(Arguments::Indices(i, j))
            }
            (false, false) => {
                let i = tape.take::<u8>()? as u16;
                let j = tape.take::<u8>()? as u16;
                Ok(Arguments::Indices(i, j))
            }
        }
    }
}

impl Default for Options {
    #[inline]
    fn default() -> Self {
        Options::None
    }
}

impl Walue<'static> for Options {
    type Parameter = ComponentFlags;

    fn read<T: Tape>(tape: &mut T, flags: ComponentFlags) -> Result<Self> {
        if flags.has_scalar_scale() {
            Ok(Options::Scalar(tape.take()?))
        } else if flags.has_vector_scale() {
            Ok(Options::Vector(tape.take()?, tape.take()?))
        } else if flags.has_matrix_scale() {
            Ok(Options::Matrix(
                tape.take()?,
                tape.take()?,
                tape.take()?,
                tape.take()?,
            ))
        } else {
            Ok(Options::None)
        }
    }
}

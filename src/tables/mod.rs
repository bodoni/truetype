//! Tables.

pub mod character_mapping;
pub mod font_header;
pub mod glyph_data;
pub mod glyph_mapping;
pub mod horizontal_header;
pub mod horizontal_metrics;
pub mod maximum_profile;
pub mod names;
pub mod offsets;
pub mod postscript;
pub mod windows_metrics;

pub use character_mapping::CharacterMapping;
pub use font_header::FontHeader;
pub use glyph_data::GlyphData;
pub use glyph_mapping::GlyphMapping;
pub use horizontal_header::HorizontalHeader;
pub use horizontal_metrics::HorizontalMetrics;
pub use maximum_profile::MaximumProfile;
pub use names::Names;
pub use offsets::Offsets;
pub use postscript::PostScript;
pub use windows_metrics::WindowsMetrics;

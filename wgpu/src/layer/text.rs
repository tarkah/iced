use crate::core::alignment;
use crate::core::text;
use crate::core::{Font, Rectangle};

/// A paragraph of text.
#[derive(Debug, Clone)]
pub struct Text<'a> {
    /// The content of the [`Text`].
    pub content: text::Content<'a, Font>,

    /// The layout bounds of the [`Text`].
    pub bounds: Rectangle,

    /// The size of the [`Text`] in logical pixels.
    pub size: f32,

    /// The line height of the [`Text`].
    pub line_height: text::LineHeight,

    /// The horizontal alignment of the [`Text`].
    pub horizontal_alignment: alignment::Horizontal,

    /// The vertical alignment of the [`Text`].
    pub vertical_alignment: alignment::Vertical,

    /// The shaping strategy of the text.
    pub shaping: text::Shaping,
}

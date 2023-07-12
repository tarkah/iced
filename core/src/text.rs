//! Draw and interact with text.
use crate::alignment;
use crate::{Color, Pixels, Point, Rectangle, Size};

use std::borrow::Cow;
use std::hash::{Hash, Hasher};

/// A paragraph.
#[derive(Debug, Clone)]
pub struct Text<'a, Font> {
    /// The content of the paragraph.
    pub content: Content<'a, Font>,

    /// The bounds of the paragraph.
    pub bounds: Rectangle,

    /// The size of the [`Text`] in logical pixels.
    pub size: f32,

    /// The line height of the [`Text`].
    pub line_height: LineHeight,

    /// The horizontal alignment of the [`Text`].
    pub horizontal_alignment: alignment::Horizontal,

    /// The vertical alignment of the [`Text`].
    pub vertical_alignment: alignment::Vertical,

    /// The [`Shaping`] strategy of the [`Text`].
    pub shaping: Shaping,
}

/// The [`Text`] content.
#[derive(Debug, Clone)]
pub enum Content<'a, Font> {
    /// A single [`Span`] of text.
    Span(Span<'a, Font>),
    /// Multiple spans of text.
    Spans(Vec<Span<'a, Font>>),
}

impl<'a, Font> Content<'a, Font> {
    /// Create a single span of text content
    pub fn span(content: &'a str, color: Color, font: Font) -> Self {
        Self::Span(Span {
            content,
            color,
            font,
        })
    }

    /// Iterate over the spans of the [`Content`].
    pub fn iter(&self) -> Box<dyn Iterator<Item = &Span<'a, Font>> + '_> {
        match self {
            Content::Span(span) => Box::new(std::iter::once(span)),
            Content::Spans(spans) => Box::new(spans.iter()),
        }
    }
}

/// A span of text.
#[derive(Debug, Clone, Copy)]
pub struct Span<'a, Font> {
    /// The content of the span.
    pub content: &'a str,

    /// The color of the [`Span`].
    pub color: Color,

    /// The font of the [`Span`].
    pub font: Font,
}

/// The shaping strategy of some text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Shaping {
    /// No shaping and no font fallback.
    ///
    /// This shaping strategy is very cheap, but it will not display complex
    /// scripts properly nor try to find missing glyphs in your system fonts.
    ///
    /// You should use this strategy when you have complete control of the text
    /// and the font you are displaying in your application.
    ///
    /// This is the default.
    #[default]
    Basic,
    /// Advanced text shaping and font fallback.
    ///
    /// You will need to enable this flag if the text contains a complex
    /// script, the font used needs it, and/or multiple fonts in your system
    /// may be needed to display all of the glyphs.
    ///
    /// Advanced shaping is expensive! You should only enable it when necessary.
    Advanced,
}

/// The height of a line of text in a paragraph.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineHeight {
    /// A factor of the size of the text.
    Relative(f32),

    /// An absolute height in logical pixels.
    Absolute(Pixels),
}

impl LineHeight {
    /// Returns the [`LineHeight`] in absolute logical pixels.
    pub fn to_absolute(self, text_size: Pixels) -> Pixels {
        match self {
            Self::Relative(factor) => Pixels(factor * text_size.0),
            Self::Absolute(pixels) => pixels,
        }
    }
}

impl Default for LineHeight {
    fn default() -> Self {
        Self::Relative(1.3)
    }
}

impl From<f32> for LineHeight {
    fn from(factor: f32) -> Self {
        Self::Relative(factor)
    }
}

impl From<Pixels> for LineHeight {
    fn from(pixels: Pixels) -> Self {
        Self::Absolute(pixels)
    }
}

impl Hash for LineHeight {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Relative(factor) => {
                state.write_u8(0);
                factor.to_bits().hash(state);
            }
            Self::Absolute(pixels) => {
                state.write_u8(1);
                f32::from(*pixels).to_bits().hash(state);
            }
        }
    }
}

/// The result of hit testing on text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hit {
    /// The point was within the bounds of the returned character index.
    CharOffset(usize),
}

impl Hit {
    /// Computes the cursor position of the [`Hit`] .
    pub fn cursor(self) -> usize {
        match self {
            Self::CharOffset(i) => i,
        }
    }
}

/// A renderer capable of measuring and drawing [`Text`].
pub trait Renderer: crate::Renderer {
    /// The font type used.
    type Font: Copy;

    /// The icon font of the backend.
    const ICON_FONT: Self::Font;

    /// The `char` representing a ✔ icon in the [`ICON_FONT`].
    ///
    /// [`ICON_FONT`]: Self::ICON_FONT
    const CHECKMARK_ICON: char;

    /// The `char` representing a ▼ icon in the built-in [`ICON_FONT`].
    ///
    /// [`ICON_FONT`]: Self::ICON_FONT
    const ARROW_DOWN_ICON: char;

    /// Returns the default [`Self::Font`].
    fn default_font(&self) -> Self::Font;

    /// Returns the default size of [`Text`].
    fn default_size(&self) -> f32;

    /// Measures the text in the given bounds and returns the minimum boundaries
    /// that can fit the contents.
    fn measure(
        &self,
        content: &Content<'_, Self::Font>,
        size: f32,
        line_height: LineHeight,
        bounds: Size,
        shaping: Shaping,
    ) -> Size;

    /// Measures the width of the text as if it were laid out in a single line.
    fn measure_width(
        &self,
        content: &Content<'_, Self::Font>,
        size: f32,
        shaping: Shaping,
    ) -> f32 {
        let bounds = self.measure(
            content,
            size,
            LineHeight::Absolute(Pixels(size)),
            Size::INFINITY,
            shaping,
        );

        bounds.width
    }

    /// Tests whether the provided point is within the boundaries of text
    /// laid out with the given parameters, returning information about
    /// the nearest character.
    ///
    /// If `nearest_only` is true, the hit test does not consider whether the
    /// the point is interior to any glyph bounds, returning only the character
    /// with the nearest centeroid.
    fn hit_test(
        &self,
        contents: &Content<'_, Self::Font>,
        size: f32,
        line_height: LineHeight,
        bounds: Size,
        shaping: Shaping,
        point: Point,
        nearest_only: bool,
    ) -> Option<Hit>;

    /// Loads a [`Self::Font`] from its bytes.
    fn load_font(&mut self, font: Cow<'static, [u8]>);

    /// Draws the given [`Text`].
    fn fill_text(&mut self, text: Text<'_, Self::Font>);
}

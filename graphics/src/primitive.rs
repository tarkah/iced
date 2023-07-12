//! Draw using different graphical primitives.
use crate::core::alignment;
use crate::core::image;
use crate::core::svg;
use crate::core::{Background, Color, Rectangle, Vector};

use std::sync::Arc;

/// A rendering primitive.
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive<T> {
    /// A text primitive
    Text {
        /// The contents of the text
        content: text::Content,
        /// The bounds of the text
        bounds: Rectangle,
        /// The size of the text in logical pixels
        size: f32,
        /// The line height of the text
        line_height: text::LineHeight,
        /// The horizontal alignment of the text
        horizontal_alignment: alignment::Horizontal,
        /// The vertical alignment of the text
        vertical_alignment: alignment::Vertical,
        /// The shaping strategy of the text.
        shaping: text::Shaping,
    },
    /// A quad primitive
    Quad {
        /// The bounds of the quad
        bounds: Rectangle,
        /// The background of the quad
        background: Background,
        /// The border radii of the quad
        border_radius: [f32; 4],
        /// The border width of the quad
        border_width: f32,
        /// The border color of the quad
        border_color: Color,
    },
    /// An image primitive
    Image {
        /// The handle of the image
        handle: image::Handle,
        /// The bounds of the image
        bounds: Rectangle,
    },
    /// An SVG primitive
    Svg {
        /// The path of the SVG file
        handle: svg::Handle,

        /// The [`Color`] filter
        color: Option<Color>,

        /// The bounds of the viewport
        bounds: Rectangle,
    },
    /// A group of primitives
    Group {
        /// The primitives of the group
        primitives: Vec<Primitive<T>>,
    },
    /// A clip primitive
    Clip {
        /// The bounds of the clip
        bounds: Rectangle,
        /// The content of the clip
        content: Box<Primitive<T>>,
    },
    /// A primitive that applies a translation
    Translate {
        /// The translation vector
        translation: Vector,

        /// The primitive to translate
        content: Box<Primitive<T>>,
    },
    /// A cached primitive.
    ///
    /// This can be useful if you are implementing a widget where primitive
    /// generation is expensive.
    Cache {
        /// The cached primitive
        content: Arc<Primitive<T>>,
    },
    /// A backend-specific primitive.
    Custom(T),
}

impl<T> Primitive<T> {
    /// Creates a [`Primitive::Group`].
    pub fn group(primitives: Vec<Self>) -> Self {
        Self::Group { primitives }
    }

    /// Creates a [`Primitive::Clip`].
    pub fn clip(self, bounds: Rectangle) -> Self {
        Self::Clip {
            bounds,
            content: Box::new(self),
        }
    }

    /// Creates a [`Primitive::Translate`].
    pub fn translate(self, translation: Vector) -> Self {
        Self::Translate {
            translation,
            content: Box::new(self),
        }
    }
}

pub mod text {
    //! Text rendering primitives.

    use crate::core::text;
    use crate::core::{Color, Font};

    pub use crate::core::text::{LineHeight, Shaping};

    /// The text content.
    #[derive(Debug, Clone, PartialEq)]
    pub enum Content {
        /// A single [`Span`] of text.
        Span(Span),
        /// Multiple spans of text.
        Spans(Vec<Span>),
    }

    impl<'a> From<text::Content<'a, Font>> for Content {
        fn from(content: text::Content<'a, Font>) -> Self {
            match content {
                text::Content::Span(span) => Content::Span(Span::from(span)),
                text::Content::Spans(spans) => {
                    Content::Spans(spans.into_iter().map(Span::from).collect())
                }
            }
        }
    }

    impl<'a> From<&'a Content> for text::Content<'a, Font> {
        fn from(content: &'a Content) -> Self {
            match content {
                Content::Span(span) => {
                    text::Content::Span(text::Span::from(span))
                }
                Content::Spans(spans) => text::Content::Spans(
                    spans.iter().map(text::Span::from).collect(),
                ),
            }
        }
    }

    /// A span of text.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Span {
        /// The content of the span.
        pub content: String,

        /// The color of the [`Span`].
        pub color: Color,

        /// The font of the [`Span`].
        pub font: Font,
    }

    impl<'a> From<text::Span<'a, Font>> for Span {
        fn from(span: text::Span<'a, Font>) -> Self {
            Self {
                content: span.content.to_string(),
                color: span.color,
                font: span.font,
            }
        }
    }

    impl<'a> From<&'a Span> for text::Span<'a, Font> {
        fn from(span: &'a Span) -> Self {
            Self {
                content: &span.content,
                color: span.color,
                font: span.font,
            }
        }
    }
}

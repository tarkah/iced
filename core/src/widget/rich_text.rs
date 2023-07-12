//! Write some rich text for your users to read.
use crate::alignment;
use crate::layout;
use crate::mouse;
use crate::renderer;
use crate::text;
use crate::widget::Tree;
use crate::{Color, Element, Layout, Length, Pixels, Rectangle, Widget};

use std::borrow::Cow;

pub use super::text::StyleSheet;
pub use text::{LineHeight, Shaping};

/// A span of text.
#[allow(missing_debug_implementations)]
pub struct Span<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// The content of the span.
    pub content: Cow<'a, str>,

    /// The style of the [`Span`].
    pub style: <Renderer::Theme as StyleSheet>::Style,

    /// The font of the [`Span`].
    pub font: Option<Renderer::Font>,
}

impl<'a, Renderer> Span<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Create a new [`Span`] with the given content.
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: content.into(),
            style: <Renderer::Theme as StyleSheet>::Style::default(),
            font: None,
        }
    }

    /// Sets the [`Font`] of the [`Span`].
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the style of the [`Span`].
    pub fn style(
        mut self,
        style: impl Into<<Renderer::Theme as StyleSheet>::Style>,
    ) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Renderer> Clone for Span<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            style: self.style.clone(),
            font: self.font,
        }
    }
}

/// A paragraph of rich text.
#[allow(missing_debug_implementations)]
pub struct RichText<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    spans: Vec<Span<'a, Renderer>>,
    size: Option<f32>,
    line_height: LineHeight,
    width: Length,
    height: Length,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    shaping: Shaping,
}

impl<'a, Renderer> RichText<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Create a new paragraph of [`RichText`] with the given spans.
    pub fn new(spans: Vec<Span<'a, Renderer>>) -> Self {
        RichText {
            spans,
            size: None,
            line_height: LineHeight::default(),
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            shaping: Shaping::Basic,
        }
    }

    /// Sets the size of the [`RichText`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().0);
        self
    }

    /// Sets the [`LineHeight`] of the [`RichText`].
    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// Sets the width of the [`RichText`] boundaries.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`RichText`] boundaries.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`alignment::Horizontal`] of the [`RichText`].
    pub fn horizontal_alignment(
        mut self,
        alignment: alignment::Horizontal,
    ) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the [`alignment::Vertical`] of the [`RichText`].
    pub fn vertical_alignment(
        mut self,
        alignment: alignment::Vertical,
    ) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Sets the [`Shaping`] strategy of the [`RichText`].
    pub fn shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = shaping;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for RichText<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let size = self.size.unwrap_or_else(|| renderer.default_size());

        let content = text::Content::Spans(
            self.spans
                .iter()
                .map(|span| text::Span {
                    content: &span.content,
                    color: Color::TRANSPARENT,
                    font: span.font.unwrap_or_else(|| renderer.default_font()),
                })
                .collect(),
        );

        let bounds = renderer.measure(
            &content,
            size,
            self.line_height,
            limits.max(),
            self.shaping,
        );

        let size = limits.resolve(bounds);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        draw(
            renderer,
            style,
            layout,
            &self.spans,
            self.size,
            self.line_height,
            theme,
            self.horizontal_alignment,
            self.vertical_alignment,
            self.shaping,
        );
    }
}

/// Draws text using the same logic as the [`RichText`] widget.
///
/// Specifically:
///
/// * If no `size` is provided, the default text size of the `Renderer` will be
///   used.
/// * If no `color` is provided by the spans, the [`renderer::Style::text_color`] will be
///   used.
/// * The alignment attributes do not affect the position of the bounds of the
///   [`Layout`].
pub fn draw<Renderer>(
    renderer: &mut Renderer,
    style: &renderer::Style,
    layout: Layout<'_>,
    spans: &[Span<'_, Renderer>],
    size: Option<f32>,
    line_height: LineHeight,
    theme: &Renderer::Theme,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    shaping: Shaping,
) where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    let bounds = layout.bounds();

    let x = match horizontal_alignment {
        alignment::Horizontal::Left => bounds.x,
        alignment::Horizontal::Center => bounds.center_x(),
        alignment::Horizontal::Right => bounds.x + bounds.width,
    };

    let y = match vertical_alignment {
        alignment::Vertical::Top => bounds.y,
        alignment::Vertical::Center => bounds.center_y(),
        alignment::Vertical::Bottom => bounds.y + bounds.height,
    };

    let size = size.unwrap_or_else(|| renderer.default_size());

    renderer.fill_text(crate::Text {
        content: text::Content::Spans(
            spans
                .iter()
                .map(|span| text::Span {
                    content: &span.content,
                    color: theme
                        .appearance(span.style.clone())
                        .color
                        .unwrap_or(style.text_color),
                    font: span.font.unwrap_or_else(|| renderer.default_font()),
                })
                .collect(),
        ),
        size,
        line_height,
        bounds: Rectangle { x, y, ..bounds },
        horizontal_alignment,
        vertical_alignment,
        shaping,
    });
}

impl<'a, Message, Renderer> From<RichText<'a, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: text::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(text: RichText<'a, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(text)
    }
}

impl<'a, Renderer> Clone for RichText<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn clone(&self) -> Self {
        Self {
            spans: self.spans.clone(),
            size: self.size,
            line_height: self.line_height,
            width: self.width,
            height: self.height,
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: self.vertical_alignment,
            shaping: self.shaping,
        }
    }
}

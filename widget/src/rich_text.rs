//! Write some rich text for your users to read.
pub use crate::core::widget::rich_text::*;

/// A paragraph.
pub type RichText<'a, Renderer = crate::Renderer> =
    crate::core::widget::RichText<'a, Renderer>;

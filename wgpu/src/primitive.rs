//! Draw using different graphical primitives.
use crate::core::Rectangle;
use crate::graphics::{Damage, Mesh};

pub use crate::graphics::primitive::text;

/// The graphical primitives supported by `iced_wgpu`.
pub type Primitive = crate::graphics::Primitive<Custom>;

/// The custom primitives supported by `iced_wgpu`.
#[derive(Debug, Clone, PartialEq)]
pub enum Custom {
    /// A mesh primitive.
    Mesh(Mesh),
}

impl Damage for Custom {
    fn bounds(&self) -> Rectangle {
        match self {
            Self::Mesh(mesh) => mesh.bounds(),
        }
    }
}

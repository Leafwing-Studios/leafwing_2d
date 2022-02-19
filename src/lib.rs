#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use crate::orientation::Rotation;
use crate::position::Position;
use bevy_ecs::bundle::Bundle;
use bevy_transform::components::{GlobalTransform, Transform};

pub mod orientation;
pub mod position;

/// The most commonly useful bits of the library
pub mod prelude {
    pub use crate::orientation::{Direction, Rotation};
    pub use crate::position::Position;
}

/// A [`Bundle`] of components that conveniently represents the state of entities living in 2-dimensional space
///
/// When used with other bundles (like a `SpriteBundle`), be aware that duplicate components (like [`Transform`])
/// will take the value of the last
#[derive(Bundle, Clone, Debug, Default)]
pub struct TwoDimBundle<T: Send + Sync + 'static> {
    /// The 2-dimensional [`Position`] of the entity
    ///
    /// This is automatically converted into a [`Transform`]'s translation
    pub position: Position<T>,
    /// Which way the entity is facing
    ///
    /// This is converted into a [`Transform`]'s rotation,
    /// And is
    pub rotation: Rotation,
    /// The local 3-dimensional position / rotation / scale of this entity
    pub transform: Transform,
    /// The absolute position / rotation / scale of this entity
    ///
    /// Can be modified by the entity's parent
    pub global_transform: GlobalTransform,
}

#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod bounding;
pub mod orientation;
pub mod plugin;
pub mod position;

/// The most commonly useful bits of the library
pub mod prelude {
    pub use crate::bounding::{AxisAlignedBoundingBox, BoundingRegion};
    pub use crate::orientation::{Direction, Orientation, Rotation};
    pub use crate::plugin::{TwoDimBundle, TwoDimPlugin};
    pub use crate::position::{discrete_coordinates::DiscreteCoordinate, Coordinate, Position};
}

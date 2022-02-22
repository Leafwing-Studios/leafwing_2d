#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod bounding;
pub mod discrete_coordinates;
pub mod errors;
pub mod kinematics;
pub mod orientation;
pub mod partitioning;
pub mod plugin;
pub mod position;

/// The most commonly useful bits of the library
pub mod prelude {
    pub use crate::bounding::{AxisAlignedBoundingBox, BoundingRegion};
    pub use crate::discrete_coordinates::DiscreteCoordinate;
    pub use crate::kinematics::{
        Acceleration, AngularAcceleration, AngularVelocity, Kinematic, Velocity,
    };
    pub use crate::orientation::{Direction, Orientation, OrientationPositionInterop, Rotation};
    pub use crate::plugin::{TwoDBundle, TwoDPlugin};
    pub use crate::position::{Coordinate, Position, Positionlike};
}

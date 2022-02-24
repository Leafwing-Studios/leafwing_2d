//! [`Coordinate`] types for [`Positions`](Position) designed for 2D games that move freely

use crate as leafwing_2d;
use crate::position::{Coordinate, TrivialCoordinate};

/// A [`f32`]-backed [`Coordinate`]
#[derive(TrivialCoordinate)]
pub struct F32(pub f32);

impl From<f32> for F32 {
    fn from(float: f32) -> F32 {
        F32(float)
    }
}

impl From<F32> for f32 {
    fn from(coordinate: F32) -> f32 {
        coordinate.0
    }
}

impl Coordinate for F32 {
    const COORD_TO_TRANSFORM: f32 = 1.0;
    const MIN: Self = f32::MIN.into();
    const MAX: Self = f32::MAX.into();
    const ZERO: Self = 0.0.into();
}

/// A [`f64`]-backed [`Coordinate`]
#[derive(TrivialCoordinate)]
pub struct F64(pub f64);

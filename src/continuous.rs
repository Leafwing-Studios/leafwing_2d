//! [`Coordinate`] types for [`Positions`](Position) designed for 2D games that move freely

use crate as leafwing_2d;
use crate::coordinate::{Coordinate, TrivialCoordinate};

/// A [`f32`]-backed [`Coordinate`]
#[derive(TrivialCoordinate)]
pub struct F32(pub f32);

impl Coordinate for F32 {
    type Data = f32;

    const COORD_TO_TRANSFORM: f32 = 1.0;
    const MIN: Self = F32(f32::MIN);
    const MAX: Self = F32(f32::MAX);
    const ZERO: Self = F32(0.0);
}

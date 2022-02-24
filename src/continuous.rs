//! [`Coordinate`] types for [`Positions`](Position) designed for 2D games that move freely

/// A [`f32`]-backed [`Coordinate`]
#[derive(Clone, Copy, Debug, PartialEq, Default, PartialOrd)]
pub struct F32(pub f32);

/// A [`f64`]-backed [`Coordinate`]
#[derive(Clone, Copy, Debug, PartialEq, Default, PartialOrd)]
pub struct F64(pub f64);

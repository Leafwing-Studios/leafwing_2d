//! 2-dimensional coordinates

use crate::bounding::{AxisAlignedBoundingBox, BoundingRegion, OrientedBoundingBox};
use crate::orientation::Direction;
use bevy_ecs::prelude::Component;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::{fmt::Debug, ops::*};

/// A 2-dimensional coordinate
///
/// The underlying data type `T` can be modified to control
/// whether the coordinate system is hexagonal vs. grid,
/// continuous or discrete and so on.
#[derive(
    Component, Default, Clone, Copy, Debug, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
)]
pub struct Position<C> {
    /// The first coordinate of the position, typically the x-axis
    pub x: C,
    /// The second coordinate of the position, typically the y-axis
    pub y: C,
}

impl<C> Position<C> {
    /// Creates a new [`Position`] with the provided `x` and `y` coordinates
    #[inline]
    #[must_use]
    pub const fn new(x: C, y: C) -> Position<C> {
        Position { x, y }
    }
}

impl<T: Sub<Output = T> + Into<f32>> Position<T> {
    /// Gets the direction that points away from this position, towards `other_position`
    #[inline]
    #[must_use]
    pub fn direction_to(self, other_position: Position<T>) -> Direction {
        let net_position: Position<T> = other_position - self;
        net_position.into()
    }

    /// Gets the direction that points towards this position, from `other_position`
    #[inline]
    #[must_use]
    pub fn direction_from(self, other_position: Position<T>) -> Direction {
        let net_position: Position<T> = self - other_position;
        net_position.into()
    }
}

/// Coordinate types for [`Position`] designed for operation on discrete grids
pub mod discrete_coordinates {
    use crate::orientation::{
        partitioning::{CardinalQuadrant, DirectionParitioning},
        Direction,
    };

    use super::*;

    /// A type that can be used to represent a discrete 2-dimensional coordinate
    ///
    /// Typically used to create and work with [`Positions`](Position)
    pub trait DiscreteCoordinate: Sized + Into<f32> {
        /// The number of neighbors
        const N_NEIGHBORS: usize;
        /// Adding or subtracting this coordinate to another coordinate does not change the value
        const ZERO: Self;
        /// The (0, 0) cell [`Position`]
        const ORIGIN: Position<Self> = Position {
            x: Self::ZERO,
            y: Self::ZERO,
        };

        /// The [`DirectionParitioning`] that determines how [`Directions`](Direction) should map to neighbors
        type Parititions: DirectionParitioning;

        /// Creates a [`Position`] from the pair of values provided
        #[must_use]
        fn position(x: Self, y: Self) -> Position<Self> {
            Position { x, y }
        }

        /// Fetches the array of neighboring [`Positions`](Position), in a fixed order
        ///
        /// The order should always be clockwise, starting from north (+y)
        #[must_use]
        fn neighbors(position: Position<Self>) -> [Position<Self>; Self::N_NEIGHBORS];

        /// The [`Direction`] towards each neighbor
        ///
        /// The order should always be clockwise, starting from north (+y)
        ///
        /// ```rust
        ///
        ///
        /// ```
        #[must_use]
        fn neighbor_directions() -> [Direction; Self::N_NEIGHBORS] {
            Self::neighbors(Self::ORIGIN).map(|position| position.into())
        }
    }

    /// [`DiscreteCoordinate`] primitive for a square grid, where each cell has four neighbors
    ///
    /// Neighboring tiles must touch on their faces
    #[derive(
        Clone,
        Copy,
        Debug,
        Add,
        AddAssign,
        Sub,
        SubAssign,
        Rem,
        RemAssign,
        Mul,
        MulAssign,
        Div,
        DivAssign,
    )]
    pub struct OrthogonalGrid(pub usize);

    impl DiscreteCoordinate for OrthogonalGrid {
        const N_NEIGHBORS: usize = 4;
        const ZERO: OrthogonalGrid = OrthogonalGrid(0);
        type Parititions = CardinalQuadrant;

        fn neighbors(position: Position<Self>) -> [Position<Self>; Self::N_NEIGHBORS] {
            [
                Position {
                    x: Self(position.x.0),
                    y: Self(position.y.0 + 1),
                },
                Position {
                    x: Self(position.x.0 + 1),
                    y: Self(position.y.0),
                },
                Position {
                    x: Self(position.x.0),
                    y: Self(position.y.0 - 1),
                },
                Position {
                    x: Self(position.x.0 - 1),
                    y: Self(position.y.0),
                },
            ]
        }
    }

    impl From<OrthogonalGrid> for f32 {
        fn from(coordinate: OrthogonalGrid) -> f32 {
            coordinate.0 as f32
        }
    }

    /// [`DiscreteCoordinate`] primitive for a square grid, where each cell has eight neighbors
    ///
    /// Neighboring tiles are a king's move away: either touching faces or diagonally adjacent
    pub struct AdjacentGrid(pub usize);

    /// [`DiscreteCoordinate`] primitive for a hexagonal grid, where each cell points sideways
    ///
    /// These hexes tile vertically, but not horizontally
    pub struct FlatHex(pub usize);

    /// [`DiscreteCoordinate`] primitive for a hexagonal grid, where each cell points up
    ///
    /// These hexes tile horizontally, but not vertically
    pub struct PointyHex(pub usize);
}

mod basic_operations {
    use super::*;

    impl<T: Add<Output = T>> Add<Position<T>> for Position<T> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl<T: Sub<Output = T>> Sub<Position<T>> for Position<T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl<T: Mul<Output = T> + Clone> Mul<T> for Position<T> {
        type Output = Position<T>;

        fn mul(self, rhs: T) -> Self::Output {
            Self {
                x: self.x * rhs.clone(),
                y: self.y * rhs,
            }
        }
    }

    impl<T: Div<Output = T> + Clone> Div<T> for Position<T> {
        type Output = Position<T>;

        fn div(self, rhs: T) -> Self::Output {
            Self {
                x: self.x / rhs.clone(),
                y: self.y / rhs,
            }
        }
    }

    impl<T: Rem<Output = T> + Clone> Rem<T> for Position<T> {
        type Output = Position<T>;

        fn rem(self, rhs: T) -> Self::Output {
            Self {
                x: self.x % rhs.clone(),
                y: self.y % rhs,
            }
        }
    }

    impl<T: Rem<Output = T>> Rem<Position<T>> for Position<T> {
        type Output = Position<T>;

        fn rem(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x % rhs.x,
                y: self.y % rhs.y,
            }
        }
    }
}

// When relevant, z-values are simply ignored
mod conversions {
    use super::*;
    use crate::orientation::Direction;
    use bevy_math::{Vec2, Vec3};
    use bevy_transform::components::{GlobalTransform, Transform};

    impl<T: From<f32>> From<Vec2> for Position<T> {
        fn from(vec: Vec2) -> Position<T> {
            Position {
                x: vec.x.into(),
                y: vec.y.into(),
            }
        }
    }

    impl<T: Into<f32>> From<Position<T>> for Vec2 {
        fn from(position: Position<T>) -> Vec2 {
            Vec2::new(position.x.into(), position.y.into())
        }
    }

    impl<T: Into<f32>> From<Position<T>> for Vec3 {
        fn from(position: Position<T>) -> Vec3 {
            Vec3::new(position.x.into(), position.y.into(), 0.0)
        }
    }

    impl<T: Into<f32>> From<Position<T>> for Direction {
        fn from(position: Position<T>) -> Direction {
            let vec2: Vec2 = position.into();

            Direction::new(vec2)
        }
    }

    impl<T: From<f32>> From<Vec3> for Position<T> {
        fn from(vec: Vec3) -> Self {
            Self {
                x: vec.x.into(),
                y: vec.y.into(),
            }
        }
    }

    impl<T: From<f32>> From<Transform> for Position<T> {
        fn from(transform: Transform) -> Self {
            Self {
                x: transform.translation.x.into(),
                y: transform.translation.y.into(),
            }
        }
    }

    impl<T: From<f32>> From<GlobalTransform> for Position<T> {
        fn from(transform: GlobalTransform) -> Self {
            Self {
                x: transform.translation.x.into(),
                y: transform.translation.y.into(),
            }
        }
    }
}

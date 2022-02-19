//! 2-dimensional coordinates

use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::{fmt::Debug, ops::*};

pub use discrete_coordinates::*;

/// A 2-dimensional coordinate
///
/// The underlying data type `T` can be modified to control
/// whether the coordinate system is hexagonal vs. grid,
/// continuous or discrete and so on.
#[derive(Default, Clone, Copy, Debug, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign)]
pub struct Position<T> {
    /// The first coordinate of the position, typically the x-axis
    pub x: T,
    /// The second coordinate of the position, typically the y-axis
    pub y: T,
}

mod discrete_coordinates {
    use super::*;

    /// A type that can be used to represent a discrete 2-dimensional coordinate
    ///
    /// Typically used to create and work with [`Positions`](Position)
    pub trait DiscreteCoordinate: Sized {
        /// The number of neighbors
        const N_NEIGHBORS: usize;

        /// Creates a [`Position`] from the pair of values provided
        fn position(x: Self, y: Self) -> Position<Self> {
            Position { x, y }
        }

        /// Fetches the array of neighboring [`Positions`](Position), in a fixed order
        ///
        /// The order should always be clockwise, starting from north (+y)
        fn neighbours(position: Position<Self>) -> [Position<Self>; Self::N_NEIGHBORS];
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

        fn neighbours(position: Position<Self>) -> [Position<Self>; Self::N_NEIGHBORS] {
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
    use bevy_math::{Vec2, Vec3};
    use bevy_transform::components::{GlobalTransform, Transform};

    impl<T: From<f32>> From<Vec2> for Position<T> {
        fn from(vec: Vec2) -> Self {
            Self {
                x: vec.x.into(),
                y: vec.y.into(),
            }
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

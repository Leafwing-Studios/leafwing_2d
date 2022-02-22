//! Coordinate types for [`Position`] designed for operation on discrete grids
//!
//! The provided types all store an `isize` under the hood for maximum flexbility.
//! If you require a different storage type,
//! please feel free to copy-paste the relevant struct def and trait impls into your game
//! and modify `isize` to your desired integer type.
use crate::orientation::Direction;
use crate::partitioning::DirectionParitioning;
use crate::position::{Coordinate, Position};
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// A type that can be used to represent a discrete 2-dimensional coordinate
///
/// Typically used to create and work with [`Positions`](Position)
pub trait DiscreteCoordinate: Coordinate {
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
        Self::neighbors(Self::ORIGIN).map(|position| {
            position
                .try_into()
                .expect("The positions of the neighbors cannot be (0,0).")
        })
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
    PartialEq,
    Default,
    PartialOrd,
)]
pub struct OrthogonalGrid(pub isize);

/*
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
*/

impl From<OrthogonalGrid> for f32 {
    fn from(coordinate: OrthogonalGrid) -> f32 {
        coordinate.0 as f32
    }
}

impl From<f32> for OrthogonalGrid {
    fn from(float: f32) -> OrthogonalGrid {
        OrthogonalGrid(float.round() as isize)
    }
}

/// [`DiscreteCoordinate`] primitive for a square grid, where each cell has eight neighbors
///
/// Neighboring tiles are a king's move away: either touching faces or diagonally adjacent
pub struct AdjacentGrid(pub isize);

/// [`DiscreteCoordinate`] primitive for a hexagonal grid, where each cell points sideways
///
/// These hexes tile vertically, but not horizontally
pub struct FlatHex(pub isize);

/// [`DiscreteCoordinate`] primitive for a hexagonal grid, where each cell points up
///
/// These hexes tile horizontally, but not vertically
pub struct PointyHex(pub isize);

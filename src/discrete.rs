//! Coordinate types for [`Position`] designed for operation on discrete grids
//!
//! The provided types all store an `isize` under the hood for maximum flexbility.
//! If you require a different storage type,
//! please feel free to copy-paste the relevant struct def and trait impls into your game
//! and modify `isize` to your desired integer type.
use crate::orientation::Direction;
use crate::partitioning::DirectionParitioning;
use crate::position::{Coordinate, Position};

/// A type that can be used to represent a discrete 2-dimensional coordinate
///
/// Typically used to create and work with [`Positions`](Position)
pub trait DiscreteCoordinate: Coordinate {
    /// The number of neighbors
    const N_NEIGHBORS: usize;

    /// The [`DirectionParitioning`] that determines how [`Directions`](Direction) should map to neighbors
    type Parititions: DirectionParitioning;

    /// Gets the next higher value
    ///
    /// Typically, this is just adding 1, bounded by [`Coordinate::MAX`].
    #[must_use]
    fn next(&self) -> Self;

    /// Get the next lower value
    ///
    /// Typically, this is just subtracting 1, bounded by [`Coordinate::MIN`].
    #[must_use]
    fn prev(&self) -> Self;

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

    /// Asserts that the values near the end of this range can be losslessly converted to and from [`f32`]
    ///
    /// If this assertion fails, your values are too tightly packed.
    /// Either decrease [`MAX`](Coordinate::MAX) and [`MIN`](Coordinate::MIN),
    /// or increase your [`COORD_TO_TRANSFORM`](Coordinate::COORD_TO_TRANSFORM) scale factor.
    fn assert_values_distinct() {
        let max_minus_one = Self::MAX.prev();
        let max_minus_two = Self::MAX.prev().prev();
        let min_plus_one = Self::MIN.next();
        let min_plus_two = Self::MIN.next().next();

        assert_eq!(Self::MAX.round_trip_coordinate_error(), Self::ZERO);
        assert_eq!(max_minus_one.round_trip_coordinate_error(), Self::ZERO);
        assert_eq!(max_minus_two.round_trip_coordinate_error(), Self::ZERO);
        assert_eq!(Self::MIN.round_trip_coordinate_error(), Self::ZERO);
        assert_eq!(min_plus_one.round_trip_coordinate_error(), Self::ZERO);
        assert_eq!(min_plus_two.round_trip_coordinate_error(), Self::ZERO);
    }
}

/// [`DiscreteCoordinate`] primitive for a square grid, where each cell has four neighbors
///
/// Neighboring tiles must touch on their faces
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

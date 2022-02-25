//! Coordinate types for [`Position`] designed for operation on discrete grids
//!
//! The provided types all store an `isize` under the hood for maximum flexbility.
//! If you require a different storage type,
//! please feel free to copy-paste the relevant struct def and trait impls into your game
//! and modify `isize` to your desired integer type.
use crate::coordinate::{Coordinate, TrivialCoordinate};
use crate::orientation::Direction;
use crate::partitioning::{CardinalQuadrant, DirectionParitioning};
use crate::position::Position;

use crate as leafwing_2d;

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
#[derive(TrivialCoordinate)]
pub struct OrthogonalGrid(pub isize);

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

impl Coordinate for OrthogonalGrid {
    type Data = isize;

    const COORD_TO_TRANSFORM: f32 = 1.;
    const ZERO: OrthogonalGrid = OrthogonalGrid(0);
    const MIN: OrthogonalGrid = OrthogonalGrid(isize::MIN);
    const MAX: OrthogonalGrid = OrthogonalGrid(isize::MAX);

    const DATA_ZERO: isize = 0;
    const DATA_ONE: isize = 1;
}

impl DiscreteCoordinate for OrthogonalGrid {
    type Parititions = CardinalQuadrant;
    const N_NEIGHBORS: usize = 4;

    #[inline]
    #[must_use]
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    #[inline]
    #[must_use]
    fn prev(&self) -> Self {
        Self(self.0 - 1)
    }

    #[inline]
    #[must_use]
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

/// [`DiscreteCoordinate`] primitive for a square grid, where each cell has eight neighbors
///
/// Neighboring tiles are a king's move away: either touching faces or diagonally adjacent
#[derive(TrivialCoordinate)]
pub struct AdjacentGrid(pub isize);

impl From<AdjacentGrid> for f32 {
    fn from(coordinate: AdjacentGrid) -> f32 {
        coordinate.0 as f32
    }
}

impl From<f32> for AdjacentGrid {
    fn from(float: f32) -> AdjacentGrid {
        AdjacentGrid(float.round() as isize)
    }
}

impl Coordinate for AdjacentGrid {
    type Data = isize;

    const COORD_TO_TRANSFORM: f32 = 1.;
    const ZERO: AdjacentGrid = AdjacentGrid(0);
    const MIN: AdjacentGrid = AdjacentGrid(isize::MIN);
    const MAX: AdjacentGrid = AdjacentGrid(isize::MAX);

    const DATA_ZERO: isize = 0;
    const DATA_ONE: isize = 1;
}

impl DiscreteCoordinate for AdjacentGrid {
    type Parititions = CardinalQuadrant;
    const N_NEIGHBORS: usize = 8;

    #[inline]
    #[must_use]
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    #[inline]
    #[must_use]
    fn prev(&self) -> Self {
        Self(self.0 - 1)
    }

    #[inline]
    #[must_use]
    fn neighbors(position: Position<Self>) -> [Position<Self>; Self::N_NEIGHBORS] {
        [
            // N
            Position {
                x: Self(position.x.0),
                y: Self(position.y.0 + 1),
            },
            // NE
            Position {
                x: Self(position.x.0 + 1),
                y: Self(position.y.0 + 1),
            },
            // E
            Position {
                x: Self(position.x.0 + 1),
                y: Self(position.y.0),
            },
            // SE
            Position {
                x: Self(position.x.0 + 1),
                y: Self(position.y.0 - 1),
            },
            // S
            Position {
                x: Self(position.x.0),
                y: Self(position.y.0 - 1),
            },
            // SW
            Position {
                x: Self(position.x.0 - 1),
                y: Self(position.y.0 - 1),
            },
            // W
            Position {
                x: Self(position.x.0 - 1),
                y: Self(position.y.0),
            },
            // NW
            Position {
                x: Self(position.x.0 - 1),
                y: Self(position.y.0 + 1),
            },
        ]
    }
}

/// [`DiscreteCoordinate`] primitive for a hexagonal grid, where each cell points sideways
///
/// These hexes tile vertically, but not horizontally
#[derive(TrivialCoordinate)]
pub struct FlatHex(pub isize);

impl From<FlatHex> for f32 {
    fn from(coordinate: FlatHex) -> f32 {
        coordinate.0 as f32
    }
}

impl From<f32> for FlatHex {
    fn from(float: f32) -> FlatHex {
        FlatHex(float.round() as isize)
    }
}

impl Coordinate for FlatHex {
    type Data = isize;

    const COORD_TO_TRANSFORM: f32 = 1.;
    const ZERO: FlatHex = FlatHex(0);
    const MIN: FlatHex = FlatHex(isize::MIN);
    const MAX: FlatHex = FlatHex(isize::MAX);

    const DATA_ZERO: isize = 0;
    const DATA_ONE: isize = 1;
}

impl DiscreteCoordinate for FlatHex {
    type Parititions = CardinalQuadrant;
    const N_NEIGHBORS: usize = 6;

    #[inline]
    #[must_use]
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    #[inline]
    #[must_use]
    fn prev(&self) -> Self {
        Self(self.0 - 1)
    }

    #[inline]
    #[must_use]
    fn neighbors(position: Position<Self>) -> [Position<Self>; Self::N_NEIGHBORS] {
        [
            // N
            Position {
                x: Self(position.x.0),
                y: Self(position.y.0 + 1),
            },
            // NE
            Position {
                x: Self(position.x.0 + 1),
                y: Self(position.y.0 + 1),
            },
            // SE
            Position {
                x: Self(position.x.0 + 1),
                y: Self(position.y.0 - 1),
            },
            // S
            Position {
                x: Self(position.x.0),
                y: Self(position.y.0 - 1),
            },
            // SW
            Position {
                x: Self(position.x.0 - 1),
                y: Self(position.y.0 - 1),
            },
            // NW
            Position {
                x: Self(position.x.0 - 1),
                y: Self(position.y.0 + 1),
            },
        ]
    }
}

/// [`DiscreteCoordinate`] primitive for a hexagonal grid, where each cell points up
///
/// These hexes tile horizontally, but not vertically
#[derive(TrivialCoordinate)]
pub struct PointyHex(pub isize);

impl From<PointyHex> for f32 {
    fn from(coordinate: PointyHex) -> f32 {
        coordinate.0 as f32
    }
}

impl From<f32> for PointyHex {
    fn from(float: f32) -> PointyHex {
        PointyHex(float.round() as isize)
    }
}

impl Coordinate for PointyHex {
    type Data = isize;

    const COORD_TO_TRANSFORM: f32 = 1.;
    const ZERO: PointyHex = PointyHex(0);
    const MIN: PointyHex = PointyHex(isize::MIN);
    const MAX: PointyHex = PointyHex(isize::MAX);

    const DATA_ZERO: isize = 0;
    const DATA_ONE: isize = 1;
}

impl DiscreteCoordinate for PointyHex {
    type Parititions = CardinalQuadrant;
    const N_NEIGHBORS: usize = 6;

    #[inline]
    #[must_use]
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    #[inline]
    #[must_use]
    fn prev(&self) -> Self {
        Self(self.0 - 1)
    }

    #[inline]
    #[must_use]
    fn neighbors(position: Position<Self>) -> [Position<Self>; Self::N_NEIGHBORS] {
        [
            // NE
            Position {
                x: Self(position.x.0 + 1),
                y: Self(position.y.0 + 1),
            },
            // E
            Position {
                x: Self(position.x.0 + 1),
                y: Self(position.y.0),
            },
            // SE
            Position {
                x: Self(position.x.0 + 1),
                y: Self(position.y.0 - 1),
            },
            // SW
            Position {
                x: Self(position.x.0 - 1),
                y: Self(position.y.0 - 1),
            },
            // W
            Position {
                x: Self(position.x.0 - 1),
                y: Self(position.y.0),
            },
            // NW
            Position {
                x: Self(position.x.0 - 1),
                y: Self(position.y.0 + 1),
            },
        ]
    }
}

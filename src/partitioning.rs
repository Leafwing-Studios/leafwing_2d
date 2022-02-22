//! Tools to partition [`Orientations`](Orientation) into discrete regions

use crate::orientation::{Direction, Orientation, Rotation};
use bevy_math::Vec2;

/// An exhaustive partitioning of the unit circle, snapping continuous directional input into one of a few possible options
///
/// Only `partitions` should be manually defined when implementing this trait for new types.
pub trait DirectionParitioning: Into<Rotation> + Into<Direction> + Into<Vec2> + Copy {
    /// Returns the vector of possible partitions that can be snapped to
    #[must_use]
    fn partitions() -> Vec<Self>;

    /// Returns a vector of the snappable rotations
    #[must_use]
    fn rotations() -> Vec<Rotation> {
        Self::partitions()
            .iter()
            .map(|&partition| partition.into())
            .collect()
    }

    /// Returns a vector of the snappable directions
    #[must_use]
    fn directions() -> Vec<Direction> {
        Self::partitions()
            .iter()
            .map(|&partition| partition.into())
            .collect()
    }

    /// Returns a vector of the snappable unit vectors
    #[must_use]
    fn unit_vectors() -> Vec<Vec2> {
        Self::partitions()
            .iter()
            .map(|&partition| partition.into())
            .collect()
    }

    /// Snaps to the nearest partition
    #[must_use]
    fn snap(rotationlike: impl Into<Rotation>) -> Self {
        let rotation = rotationlike.into();

        Self::partitions()
            .iter()
            .map(|&partition| (partition, rotation.distance(partition.into())))
            .reduce(|(paritition_1, distance_1), (partition_2, distance_2)| {
                // Return the closest distance from the entire set of possibilities
                if distance_1 < distance_2 {
                    (paritition_1, distance_1)
                } else {
                    (partition_2, distance_2)
                }
            })
            .expect(
                "At least one element must be returned by `DirectionPartitioning::partitions()`",
            )
            .0
    }

    /// Snaps a [`Rotation`] to the nearest matching discrete [`Rotation`]
    #[must_use]
    fn snap_rotation(rotation: Rotation) -> Rotation {
        Self::snap(rotation).into()
    }

    /// Snaps a [`Direction`] to the nearest matching discrete [`Direction`]
    #[must_use]
    fn snap_direction(direction: Direction) -> Direction {
        Self::snap_rotation(direction.into()).into()
    }

    /// Snaps a [`Vec2`] to the nearest matching discrete [`Direction`], preserving the magnitude
    ///
    /// If `vec2` has zero length, `Vec2::ZERO` will be returned instead.
    #[must_use]
    fn snap_vec2(vec2: Vec2) -> Vec2 {
        if let Ok(rotation) = vec2.try_into() {
            Self::snap_rotation(rotation).into()
        } else {
            Vec2::ZERO
        }
    }
}

/// A 4-way [`DirectionParitioning`], corresponding to the four cardinal directions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardinalQuadrant {
    /// Up
    North,
    /// Right
    East,
    /// Down
    South,
    /// Left
    West,
}

impl DirectionParitioning for CardinalQuadrant {
    fn partitions() -> Vec<Self> {
        use CardinalQuadrant::*;

        vec![North, East, South, West]
    }
}

/// A 4-way [`DirectionParitioning`], corresponding to the four cardinal directions offset by 45 degrees
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffsetQuadrant {
    /// Up and right
    NorthEast,
    /// Down and right
    SouthEast,
    /// Down and left
    SouthWest,
    /// Up and left
    NorthWest,
}

impl DirectionParitioning for OffsetQuadrant {
    fn partitions() -> Vec<Self> {
        use OffsetQuadrant::*;

        vec![NorthEast, SouthEast, SouthWest, NorthWest]
    }
}

/// A 8-way [`DirectionParitioning`], corresponding to the four cardinal directions and the intermediate values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardinalOctant {
    /// Up
    North,
    /// Up and right
    NorthEast,
    /// Right
    East,
    /// Down and right
    SouthEast,
    /// Down
    South,
    /// Down and left
    SouthWest,
    /// Left
    West,
    /// Up and left
    NorthWest,
}

impl DirectionParitioning for CardinalOctant {
    fn partitions() -> Vec<Self> {
        use CardinalOctant::*;

        vec![
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ]
    }
}

/// A 6-way [`DirectionParitioning`], corresponding to the 6 directions of a tip-up hexagon
///
/// For visualization purposes, these hexagons can be tiled in a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum CardinalSextant {
    /// Up
    North,
    /// Up and right
    NorthEast,
    /// Down and right
    SouthEast,
    /// Down
    South,
    /// Down and left
    SouthWest,
    /// Up and left
    NorthWest,
}

impl DirectionParitioning for CardinalSextant {
    fn partitions() -> Vec<Self> {
        use CardinalSextant::*;

        vec![North, NorthEast, SouthEast, South, SouthWest, NorthWest]
    }
}

/// A 6-way [`DirectionParitioning`], corresponding to the 6 directions of a flat-up hexagon
///
/// For visualization purposes, these hexagons can be tiled in a column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum OffsetSextant {
    /// Up and right
    NorthEast,
    /// Right
    East,
    /// Down and right
    SouthEast,
    /// Down and left,
    SouthWest,
    /// Left
    West,
    /// Up and left
    NorthWest,
}

impl DirectionParitioning for OffsetSextant {
    fn partitions() -> Vec<Self> {
        use OffsetSextant::*;

        vec![NorthEast, East, SouthEast, SouthWest, West, NorthWest]
    }
}

mod parition_conversions {
    use super::*;

    // Quadrant
    impl From<CardinalQuadrant> for Rotation {
        fn from(quadrant: CardinalQuadrant) -> Rotation {
            match quadrant {
                CardinalQuadrant::North => Rotation::from_degrees(0.0),
                CardinalQuadrant::East => Rotation::from_degrees(90.0),
                CardinalQuadrant::South => Rotation::from_degrees(180.0),
                CardinalQuadrant::West => Rotation::from_degrees(270.0),
            }
        }
    }

    impl From<CardinalQuadrant> for Direction {
        fn from(quadrant: CardinalQuadrant) -> Direction {
            let rotation: Rotation = quadrant.into();
            rotation.into()
        }
    }

    impl From<CardinalQuadrant> for Vec2 {
        fn from(quadrant: CardinalQuadrant) -> Vec2 {
            let rotation: Rotation = quadrant.into();
            rotation.into()
        }
    }

    // Quadrant
    impl From<OffsetQuadrant> for Rotation {
        fn from(quadrant: OffsetQuadrant) -> Rotation {
            match quadrant {
                OffsetQuadrant::NorthEast => Rotation::from_degrees(45.0),
                OffsetQuadrant::SouthEast => Rotation::from_degrees(135.0),
                OffsetQuadrant::SouthWest => Rotation::from_degrees(225.0),
                OffsetQuadrant::NorthWest => Rotation::from_degrees(315.0),
            }
        }
    }

    impl From<OffsetQuadrant> for Direction {
        fn from(quadrant: OffsetQuadrant) -> Direction {
            let rotation: Rotation = quadrant.into();
            rotation.into()
        }
    }

    impl From<OffsetQuadrant> for Vec2 {
        fn from(quadrant: OffsetQuadrant) -> Vec2 {
            let rotation: Rotation = quadrant.into();
            rotation.into()
        }
    }

    // Octant
    impl From<CardinalOctant> for Rotation {
        fn from(octant: CardinalOctant) -> Rotation {
            match octant {
                CardinalOctant::North => Rotation::from_degrees(0.0),
                CardinalOctant::NorthEast => Rotation::from_degrees(45.0),
                CardinalOctant::East => Rotation::from_degrees(90.0),
                CardinalOctant::SouthEast => Rotation::from_degrees(135.0),
                CardinalOctant::South => Rotation::from_degrees(180.0),
                CardinalOctant::SouthWest => Rotation::from_degrees(225.0),
                CardinalOctant::West => Rotation::from_degrees(270.0),
                CardinalOctant::NorthWest => Rotation::from_degrees(315.0),
            }
        }
    }

    impl From<CardinalOctant> for Direction {
        fn from(octant: CardinalOctant) -> Direction {
            let rotation: Rotation = octant.into();
            rotation.into()
        }
    }

    impl From<CardinalOctant> for Vec2 {
        fn from(octant: CardinalOctant) -> Vec2 {
            let rotation: Rotation = octant.into();
            rotation.into()
        }
    }

    // Sextant
    impl From<CardinalSextant> for Rotation {
        fn from(sextant: CardinalSextant) -> Rotation {
            match sextant {
                CardinalSextant::North => Rotation::from_degrees(0.0),
                CardinalSextant::NorthEast => Rotation::from_degrees(60.0),
                CardinalSextant::SouthEast => Rotation::from_degrees(120.0),
                CardinalSextant::South => Rotation::from_degrees(180.0),
                CardinalSextant::SouthWest => Rotation::from_degrees(240.0),
                CardinalSextant::NorthWest => Rotation::from_degrees(300.0),
            }
        }
    }

    impl From<CardinalSextant> for Direction {
        fn from(sextant: CardinalSextant) -> Direction {
            let rotation: Rotation = sextant.into();
            rotation.into()
        }
    }

    impl From<CardinalSextant> for Vec2 {
        fn from(sextant: CardinalSextant) -> Vec2 {
            let rotation: Rotation = sextant.into();
            rotation.into()
        }
    }

    // OffsetSextant
    impl From<OffsetSextant> for Rotation {
        fn from(sextant: OffsetSextant) -> Rotation {
            match sextant {
                OffsetSextant::NorthEast => Rotation::from_degrees(30.0),
                OffsetSextant::East => Rotation::from_degrees(90.0),
                OffsetSextant::SouthEast => Rotation::from_degrees(150.0),
                OffsetSextant::SouthWest => Rotation::from_degrees(210.0),
                OffsetSextant::West => Rotation::from_degrees(270.0),
                OffsetSextant::NorthWest => Rotation::from_degrees(330.0),
            }
        }
    }

    impl From<OffsetSextant> for Direction {
        fn from(sextant: OffsetSextant) -> Direction {
            let rotation: Rotation = sextant.into();
            rotation.into()
        }
    }

    impl From<OffsetSextant> for Vec2 {
        fn from(sextant: OffsetSextant) -> Vec2 {
            let rotation: Rotation = sextant.into();
            rotation.into()
        }
    }
}

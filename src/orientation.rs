//! Direction and rotation for spinning around in 2 dimensions

use bevy_math::Vec2;
use derive_more::{Display, Error};

pub use direction::Direction;
pub use orientation_position_trait::OrientationPositionInterop;
pub use orientation_trait::Orientation;
pub use rotation::Rotation;

mod orientation_trait {
    use super::{Direction, Rotation, RotationDirection};
    use bevy_math::Quat;
    use core::fmt::Debug;

    /// A type that can represent a orientation in 2D space
    pub trait Orientation: Sized + Debug + From<Rotation> + Into<Rotation> + Copy {
        /// Returns the absolute distance between `self` and `other` as a [`Rotation`]
        ///
        /// The shortest path will always be taken, and so this value ranges between 0 and 180 degrees.
        /// Simply subtract the two rotations if you want a signed value instead.
        ///
        /// # Example
        /// ```rust
        /// use leafwing_2d::orientation::{Orientation, Direction, Rotation};
        ///
        /// Direction::NORTH.distance(Direction::SOUTHWEST).assert_approx_eq(Rotation::from_degrees(135.));
        /// ```
        #[must_use]
        fn distance(&self, other: Self) -> Rotation;

        /// Asserts that `self` is approximately equal to `other`
        ///
        /// The tolerance is 2 deci-degrees.
        fn assert_approx_eq(&self, other: Self) {
            let distance: Rotation = self.distance(other);
            dbg!(self);
            dbg!(other);
            assert!(distance <= Rotation::new(2));
        }

        /// Which [`RotationDirection`] is the shortest to rotate towards to reach `target`?
        ///
        /// # Example
        /// ```rust
        /// use leafwing_2d::orientation::{Direction, Orientation, RotationDirection};
        ///
        /// assert_eq!(Direction::WEST.rotation_direction(Direction::SOUTHEAST), RotationDirection::CounterClockwise);
        /// assert_eq!(Direction::SOUTHEAST.rotation_direction(Direction::WEST), RotationDirection::Clockwise);
        /// ```
        #[inline]
        #[must_use]
        fn rotation_direction(&self, target: Self) -> RotationDirection {
            let self_rotation: Rotation = (*self).into();
            let target_rotation: Rotation = target.into();

            let initial_distance = if self_rotation.deci_degrees >= target_rotation.deci_degrees {
                self_rotation.deci_degrees - target_rotation.deci_degrees
            } else {
                target_rotation.deci_degrees - self_rotation.deci_degrees
            };

            if initial_distance <= Rotation::FULL_CIRCLE / 2 {
                RotationDirection::Clockwise
            } else {
                RotationDirection::CounterClockwise
            }
        }

        /// Rotates `self` towards `target_orientation` by up to `max_rotation`
        #[inline]
        fn rotate_towards_orientation(
            &mut self,
            target_orientation: Self,
            max_rotation: Option<Rotation>,
        ) {
            if let Some(max_rotation) = max_rotation {
                if self.distance(target_orientation) <= max_rotation {
                    *self = target_orientation;
                } else {
                    let delta_rotation = match self.rotation_direction(target_orientation) {
                        RotationDirection::Clockwise => max_rotation,
                        RotationDirection::CounterClockwise => -max_rotation,
                    };
                    let current_rotation: Rotation = (*self).into();
                    let new_rotation: Rotation = current_rotation + delta_rotation;

                    *self = new_rotation.into();
                }
            } else {
                *self = target_orientation;
            }
        }
    }

    impl Orientation for Rotation {
        #[inline]
        fn distance(&self, other: Rotation) -> Rotation {
            let initial_distance = if self.deci_degrees >= other.deci_degrees {
                self.deci_degrees - other.deci_degrees
            } else {
                other.deci_degrees - self.deci_degrees
            };

            if initial_distance <= Rotation::FULL_CIRCLE / 2 {
                Rotation {
                    deci_degrees: initial_distance,
                }
            } else {
                Rotation {
                    deci_degrees: Rotation::FULL_CIRCLE - initial_distance,
                }
            }
        }
    }

    impl Orientation for Direction {
        fn distance(&self, other: Direction) -> Rotation {
            let self_rotation: Rotation = (*self).into();
            let other_rotation: Rotation = other.into();
            self_rotation.distance(other_rotation)
        }
    }

    impl Orientation for Quat {
        fn distance(&self, other: Quat) -> Rotation {
            let self_rotation: Rotation = (*self).into();
            let other_rotation: Rotation = other.into();
            self_rotation.distance(other_rotation)
        }
    }
}

mod orientation_position_trait {
    use crate::orientation::{NearlySingularConversion, Orientation, Rotation};
    use crate::position::{Coordinate, Position};

    /// Tools that require both a [`Positions`](Position) and an [`Orientations`](Orientation)
    ///
    /// This trait is automatically implemented for all types that meet its bounds.
    /// This trait is distinct from [`Orientation`] to avoid polluting it with the generic `C`.
    pub trait OrientationPositionInterop<C: Coordinate>:
        Orientation + TryFrom<Position<C>, Error = NearlySingularConversion>
    {
        /// Computes the orientation facing from `position_a` to `position_b`
        #[inline]
        fn orientation_to_position(
            position_a: Position<C>,
            position_b: Position<C>,
        ) -> Result<Self, NearlySingularConversion> {
            let net_position: Position<C> = position_b - position_a;
            net_position.try_into()
        }

        /// Rotates `self` towards `target_position` by up to `max_rotation`
        #[inline]
        fn rotate_towards_position(
            &mut self,
            current_position: Position<C>,
            target_position: Position<C>,
            max_rotation: Option<Rotation>,
        ) {
            if let Ok(target_orientation) =
                Self::orientation_to_position(current_position, target_position)
            {
                self.rotate_towards_orientation(target_orientation, max_rotation);
            }
        }
    }

    impl<
            C: Coordinate,
            T: Orientation + TryFrom<Position<C>, Error = NearlySingularConversion>,
        > OrientationPositionInterop<C> for T
    {
    }
}

/// The supplied vector-like struct was too close to zero to be converted into a rotation-like type
///
/// This error is produced when attempting to convert into a rotation-like type
/// such as a [`Rotation`] or [`Quat`](bevy::math::Quat) from a vector-like type
/// such as a [`Vec2`].
///
/// In almost all cases, the correct way to handle this error is to simply not change the rotation.
#[derive(Debug, Clone, Copy, Error, Display, PartialEq, Eq)]
pub struct NearlySingularConversion;

/// A direction that a [`Rotation`] can be applied in
///
/// # Example
/// ```rust
/// use leafwing_2d::orientation::{Orientation, Rotation, RotationDirection};
///
/// assert_eq!(Rotation::NORTH.rotation_direction(Rotation::NORTH), RotationDirection::Clockwise);
/// assert_eq!(Rotation::NORTH.rotation_direction(Rotation::EAST), RotationDirection::Clockwise);
/// assert_eq!(Rotation::NORTH.rotation_direction(Rotation::WEST), RotationDirection::CounterClockwise);
/// assert_eq!(Rotation::NORTH.rotation_direction(Rotation::SOUTH), RotationDirection::Clockwise);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationDirection {
    /// Corresponds to a positive rotation
    Clockwise,
    /// Corresponds to a negative rotation
    CounterClockwise,
}

impl Default for RotationDirection {
    fn default() -> RotationDirection {
        RotationDirection::Clockwise
    }
}

mod rotation {
    use super::NearlySingularConversion;
    use bevy_ecs::prelude::Component;
    use bevy_math::Vec2;
    use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

    /// A discretized 2-dimensional rotation
    ///
    /// Internally, these are stored in normalized tenths of a degree, and so can be cleanly added and reversed
    /// without accumulating error.
    ///
    /// # Example
    /// ```rust
    /// use leafwing_2d::orientation::{Rotation, Direction, Orientation};
    /// use core::f32::consts::{PI, TAU};
    ///
    /// let three_o_clock = Rotation::from_degrees(90.0);
    /// let six_o_clock = Rotation::from_radians(PI);
    /// let nine_o_clock = Rotation::from_degrees(-90.0);
    ///
    /// Rotation::default().assert_approx_eq(Rotation::from_radians(0.0));
    /// Rotation::default().assert_approx_eq(Rotation::from_radians(TAU));
    /// Rotation::default().assert_approx_eq(500.0 * Rotation::from_radians(TAU));
    ///
    /// (three_o_clock + six_o_clock).assert_approx_eq(nine_o_clock);
    /// (nine_o_clock - three_o_clock).assert_approx_eq(six_o_clock);
    /// (2.0 * nine_o_clock).assert_approx_eq(six_o_clock);
    /// (six_o_clock / 2.0).assert_approx_eq(three_o_clock);
    ///
    /// six_o_clock.assert_approx_eq(Rotation::SOUTH);
    ///
    /// Direction::from(nine_o_clock).assert_approx_eq(Direction::WEST);
    /// ```
    #[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Default)]
    pub struct Rotation {
        /// Tenths of a degree, measured clockwise from midnight (x=0, y=1)
        ///
        /// 3600 make up a full circle.
        pub(crate) deci_degrees: u16,
    }

    // Useful methods
    impl Rotation {
        /// Creates a new [`Rotation`] from a whole number of tenths of a degree
        ///
        /// Measured clockwise from midnight.
        #[inline]
        #[must_use]
        pub const fn new(deci_degrees: u16) -> Rotation {
            Rotation {
                deci_degrees: deci_degrees % Rotation::FULL_CIRCLE,
            }
        }

        /// Returns the exact internal mesaurement, stored in tenths of a degree
        ///
        /// Measured clockwise from midnight (x=0, y=1).
        /// 3600 make up a full circle.
        #[inline]
        #[must_use]
        pub const fn deci_degrees(&self) -> u16 {
            self.deci_degrees
        }
    }

    // Constants
    impl Rotation {
        /// The number of deci-degrees that make up a full circle
        pub const FULL_CIRCLE: u16 = 3600;

        /// The direction that points straight up
        pub const NORTH: Rotation = Rotation { deci_degrees: 0 };

        /// The direction that points straight right
        pub const EAST: Rotation = Rotation { deci_degrees: 900 };
        /// The direction that points straight down
        pub const SOUTH: Rotation = Rotation { deci_degrees: 1800 };
        /// The direction that points straight left
        pub const WEST: Rotation = Rotation { deci_degrees: 2700 };

        /// The direction that points halfway between up and right
        pub const NORTHEAST: Rotation = Rotation { deci_degrees: 450 };
        /// The direction that points halfway between down and right
        pub const SOUTHEAST: Rotation = Rotation { deci_degrees: 1350 };
        /// The direction that points halfway between down and left
        pub const SOUTHWEST: Rotation = Rotation { deci_degrees: 2250 };
        /// The direction that points halfway between left and up
        pub const NORTHWEST: Rotation = Rotation { deci_degrees: 3150 };
    }

    // Conversion methods
    impl Rotation {
        /// Constructs a [`Rotation`](crate::orientation::Direction) from an (x,y) Euclidean coordinate
        ///
        /// If both x and y are nearly 0 (the magnitude is less than [`EPSILON`](f32::EPSILON)),
        /// [`Err(NearlySingularConversion)`] will be returned instead.
        ///
        /// # Example
        /// ```rust
        /// use bevy_math::Vec2;
        /// use leafwing_2d::orientation::Rotation;
        ///
        /// assert_eq!(Rotation::from_xy(Vec2::new(0.0, 1.0)), Ok(Rotation::NORTH));
        /// ```
        #[inline]
        pub fn from_xy(xy: Vec2) -> Result<Rotation, NearlySingularConversion> {
            if xy.length_squared() < f32::EPSILON * f32::EPSILON {
                Err(NearlySingularConversion)
            } else {
                let radians = f32::atan2(xy.x, xy.y);
                Ok(Rotation::from_radians(radians))
            }
        }

        /// Converts this direction into an (x, y) pair with magnitude 1
        #[inline]
        #[must_use]
        pub fn into_xy(self) -> Vec2 {
            let radians = self.into_radians();
            Vec2::new(radians.sin(), radians.cos())
        }

        /// Construct a [`Direction`](crate::orientation::Direction) from radians, measured clockwise from midnight
        #[must_use]
        #[inline]
        pub fn from_radians(radians: impl Into<f32>) -> Rotation {
            use std::f32::consts::TAU;

            let normalized_radians: f32 = radians.into().rem_euclid(TAU);

            Rotation {
                deci_degrees: (normalized_radians * 3600. / TAU) as u16,
            }
        }

        /// Converts this direction into radians, measured clockwise from midnight
        #[inline]
        #[must_use]
        pub fn into_radians(self) -> f32 {
            self.deci_degrees as f32 * std::f32::consts::TAU / 3600.
        }

        /// Construct a [`Direction`](crate::orientation::Direction) from degrees, measured clockwise from midnight
        #[must_use]
        #[inline]
        pub fn from_degrees(degrees: impl Into<f32>) -> Rotation {
            let normalized_degrees: f32 = degrees.into().rem_euclid(360.0);

            Rotation {
                deci_degrees: (normalized_degrees * 10.0) as u16,
            }
        }

        /// Converts this direction into degrees, measured clockwise from midnight
        #[inline]
        #[must_use]
        pub fn into_degrees(self) -> f32 {
            self.deci_degrees as f32 / 10.
        }
    }

    impl Add for Rotation {
        type Output = Rotation;
        fn add(self, rhs: Self) -> Rotation {
            Rotation::new(self.deci_degrees + rhs.deci_degrees)
        }
    }

    impl Sub for Rotation {
        type Output = Rotation;
        fn sub(self, rhs: Self) -> Rotation {
            if self.deci_degrees >= rhs.deci_degrees {
                Rotation::new(self.deci_degrees - rhs.deci_degrees)
            } else {
                Rotation::new(self.deci_degrees + Rotation::FULL_CIRCLE - rhs.deci_degrees)
            }
        }
    }

    impl AddAssign for Rotation {
        fn add_assign(&mut self, rhs: Self) {
            self.deci_degrees = (self.deci_degrees + rhs.deci_degrees) % Rotation::FULL_CIRCLE;
        }
    }

    impl SubAssign for Rotation {
        fn sub_assign(&mut self, rhs: Self) {
            self.deci_degrees = (self.deci_degrees - rhs.deci_degrees) % Rotation::FULL_CIRCLE;
        }
    }

    impl Neg for Rotation {
        type Output = Rotation;
        fn neg(self) -> Rotation {
            Rotation {
                deci_degrees: Rotation::FULL_CIRCLE - self.deci_degrees,
            }
        }
    }

    impl Mul<f32> for Rotation {
        type Output = Rotation;
        fn mul(self, rhs: f32) -> Rotation {
            Rotation::from_degrees(self.into_degrees() * rhs)
        }
    }

    impl Mul<Rotation> for f32 {
        type Output = Rotation;
        fn mul(self, rhs: Rotation) -> Rotation {
            Rotation::from_degrees(rhs.into_degrees() * self)
        }
    }

    impl Div<f32> for Rotation {
        type Output = Rotation;
        fn div(self, rhs: f32) -> Rotation {
            Rotation::from_degrees(self.into_degrees() / rhs)
        }
    }

    impl Div<Rotation> for f32 {
        type Output = Rotation;
        fn div(self, rhs: Rotation) -> Rotation {
            Rotation::from_degrees(self / rhs.into_degrees())
        }
    }
}

mod direction {
    use bevy_ecs::prelude::Component;
    use bevy_math::{const_vec2, Vec2, Vec3};
    use core::ops::{Add, Div, Mul, Neg, Sub};
    use std::f32::consts::SQRT_2;

    /// A 2D unit vector that represents a direction
    ///
    /// Its magnitude is always one.
    ///
    /// # Example
    /// ```rust
    /// use leafwing_2d::orientation::Direction;
    /// use bevy::math::Vec2;
    ///
    /// assert_eq!(Direction::NORTH.unit_vector(), Vec2::new(0.0, 1.0));
    /// assert_eq!(Direction::try_from(Vec2::ONE), Ok(Direction::NORTHEAST));
    ///
    /// assert_eq!(Direction::SOUTH * 3.0, Vec2::new(0.0, -3.0));
    /// assert_eq!(Direction::EAST / 2.0, Vec2::new(0.5, 0.0));
    /// ```
    #[derive(Component, Clone, Copy, Debug, PartialEq)]
    pub struct Direction {
        pub(crate) unit_vector: Vec2,
    }

    impl Default for Direction {
        /// [`Direction::NORTH`] is the default direction,
        /// as it is consistent with the default [`Rotation`]
        fn default() -> Direction {
            Direction::NORTH
        }
    }

    impl Direction {
        /// Creates a new [`Direction`] from a [`Vec2`]
        ///
        /// The [`Vec2`] will be normalized to have a magnitude of 1.
        ///
        /// # Panics
        /// Panics if the supplied vector has length zero.
        #[must_use]
        #[inline]
        pub fn new(vec2: Vec2) -> Self {
            if vec2.length_squared() == 0.0 {
                panic!("Supplied a Vec2 with length 0 to a Direction.")
            };

            Self {
                unit_vector: vec2.normalize(),
            }
        }

        /// Returns the raw underlying [`Vec2`] unit vector of this direction
        ///
        /// This will always have a magnitude of 1, unless it is [`Direction::NEUTRAL`]
        #[must_use]
        #[inline]
        pub const fn unit_vector(&self) -> Vec2 {
            self.unit_vector
        }
    }

    // Constants
    impl Direction {
        /// The direction that points straight up
        pub const NORTH: Direction = Direction {
            unit_vector: const_vec2!([0.0, 1.0]),
        };
        /// The direction that points straight right
        pub const EAST: Direction = Direction {
            unit_vector: const_vec2!([1.0, 0.0]),
        };
        /// The direction that points straight down
        pub const SOUTH: Direction = Direction {
            unit_vector: const_vec2!([0.0, -1.0]),
        };
        /// The direction that points straight left
        pub const WEST: Direction = Direction {
            unit_vector: const_vec2!([-1.0, 0.0]),
        };

        /// The direction that points halfway between up and right
        pub const NORTHEAST: Direction = Direction {
            unit_vector: const_vec2!([SQRT_2 / 2.0, SQRT_2 / 2.0]),
        };
        /// The direction that points halfway between down and right
        pub const SOUTHEAST: Direction = Direction {
            unit_vector: const_vec2!([SQRT_2 / 2.0, -SQRT_2 / 2.0]),
        };
        /// The direction that points halfway between down and left
        pub const SOUTHWEST: Direction = Direction {
            unit_vector: const_vec2!([-SQRT_2 / 2.0, -SQRT_2 / 2.0]),
        };
        /// The direction that points halfway between left and up
        pub const NORTHWEST: Direction = Direction {
            unit_vector: const_vec2!([-SQRT_2 / 2.0, SQRT_2 / 2.0]),
        };
    }

    impl Add for Direction {
        type Output = Vec2;
        fn add(self, other: Direction) -> Vec2 {
            self.unit_vector + other.unit_vector
        }
    }

    impl Sub for Direction {
        type Output = Vec2;

        fn sub(self, rhs: Direction) -> Vec2 {
            self.unit_vector - rhs.unit_vector
        }
    }

    impl Mul<f32> for Direction {
        type Output = Vec2;

        fn mul(self, rhs: f32) -> Self::Output {
            Vec2::new(self.unit_vector.x * rhs, self.unit_vector.y * rhs)
        }
    }

    impl Mul<Direction> for f32 {
        type Output = Vec2;

        fn mul(self, rhs: Direction) -> Self::Output {
            Vec2::new(self * rhs.unit_vector.x, self * rhs.unit_vector.y)
        }
    }

    impl Div<f32> for Direction {
        type Output = Vec2;

        fn div(self, rhs: f32) -> Self::Output {
            Vec2::new(self.unit_vector.x / rhs, self.unit_vector.y / rhs)
        }
    }

    impl Div<Direction> for f32 {
        type Output = Vec2;

        fn div(self, rhs: Direction) -> Self::Output {
            Vec2::new(self / rhs.unit_vector.x, self / rhs.unit_vector.y)
        }
    }

    impl From<Direction> for Vec3 {
        fn from(direction: Direction) -> Vec3 {
            Vec3::new(direction.unit_vector.x, direction.unit_vector.y, 0.0)
        }
    }

    impl Neg for Direction {
        type Output = Self;

        fn neg(self) -> Self {
            Self {
                unit_vector: -self.unit_vector,
            }
        }
    }
}

mod conversions {
    use super::{Direction, NearlySingularConversion, Rotation};
    use bevy_math::{Quat, Vec2, Vec3};

    impl From<Rotation> for Direction {
        fn from(rotation: Rotation) -> Direction {
            Direction {
                unit_vector: rotation.into_xy(),
            }
        }
    }

    impl From<Direction> for Rotation {
        fn from(direction: Direction) -> Rotation {
            let radians = f32::atan2(direction.unit_vector().x, direction.unit_vector().y);
            Rotation::from_radians(radians)
        }
    }

    impl TryFrom<Vec2> for Rotation {
        type Error = NearlySingularConversion;

        fn try_from(vec2: Vec2) -> Result<Rotation, NearlySingularConversion> {
            Rotation::from_xy(vec2)
        }
    }

    impl From<Rotation> for Vec2 {
        fn from(rotation: Rotation) -> Vec2 {
            rotation.into_xy()
        }
    }

    impl TryFrom<Vec2> for Direction {
        type Error = NearlySingularConversion;

        fn try_from(vec2: Vec2) -> Result<Direction, NearlySingularConversion> {
            if vec2.length_squared() == 0.0 {
                Err(NearlySingularConversion)
            } else {
                Ok(Direction {
                    unit_vector: vec2.normalize(),
                })
            }
        }
    }

    impl From<Direction> for Vec2 {
        fn from(direction: Direction) -> Vec2 {
            direction.unit_vector()
        }
    }

    impl From<Quat> for Rotation {
        fn from(quaternion: Quat) -> Rotation {
            let direction: Direction = quaternion.into();
            direction.into()
        }
    }

    impl From<Rotation> for Quat {
        fn from(rotation: Rotation) -> Self {
            Quat::from_rotation_z(rotation.into_radians())
        }
    }

    impl From<Quat> for Direction {
        fn from(quaternion: Quat) -> Self {
            // +Y is the default direction, we need to rotate it by the quaternion
            // in order to receive the new direction
            // We must use the inverse quaternion to ensure that the rotation is applied in the correct direction.
            let vec2 = quaternion.inverse().mul_vec3(Vec3::Y).truncate();

            if vec2 == Vec2::ZERO {
                Direction::default()
            } else {
                Direction {
                    unit_vector: vec2.normalize(),
                }
            }
        }
    }

    impl From<Direction> for Quat {
        fn from(direction: Direction) -> Quat {
            let rotation: Rotation = direction.into();
            rotation.into()
        }
    }
}

/// Tools to partition directions into discrete regions
pub mod partitioning {
    use super::*;

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
}

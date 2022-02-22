//! 2-dimensional coordinates

pub use coordinate::Coordinate;
pub use position_struct::Position;
pub use positionlike::Positionlike;

mod position_struct {
    use super::Coordinate;
    use crate::errors::NearlySingularConversion;
    use crate::orientation::OrientationPositionInterop;
    use bevy_ecs::prelude::Component;
    use derive_more::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

    /// A 2-dimensional coordinate
    ///
    /// The underlying data type `T` can be modified to control
    /// whether the coordinate system is hexagonal vs. grid,
    /// continuous or discrete and so on.
    ///
    /// # Example
    /// ```rust
    /// use leafwing_2d::position::Position;
    /// let origin = Position::default();
    /// let player_position = Position::<f32>::new(10.0, 4.0);
    ///
    /// assert_eq!(player_position + origin, player_position);
    /// assert_eq!(player_position - origin, player_position);
    /// ```
    #[derive(
        Component,
        Default,
        Clone,
        Copy,
        Debug,
        AddAssign,
        SubAssign,
        MulAssign,
        DivAssign,
        RemAssign,
        PartialEq,
    )]
    pub struct Position<C: Coordinate> {
        /// The first coordinate of the position, typically the x-axis
        pub x: C,
        /// The second coordinate of the position, typically the y-axis
        pub y: C,
    }

    impl<C: Coordinate> Position<C> {
        /// Creates a new [`Position`] with the provided `x` and `y` coordinates
        #[inline]
        #[must_use]
        pub fn new(x: C, y: C) -> Position<C> {
            Position { x, y }
        }
    }

    impl<C: Coordinate> Position<C> {
        /// Gets the [`Orientation`](crate::orientation::Orientation) that points away from this position towards `other_position`
        ///
        /// # Example
        /// ```rust
        /// use leafwing_2d::position::Position;
        /// use leafwing_2d::orientation::{Rotation, Orientation};
        ///
        ///
        /// let origin = Position::<f32>::default();
        /// let target = Position::new(0.0, 1.0);
        ///
        /// let rotation: Rotation = origin.orientation_to(target).expect("These positions are distinct.");
        /// rotation.assert_approx_eq(Rotation::NORTH);
        /// ```
        #[inline]
        pub fn orientation_to<O: OrientationPositionInterop<C>>(
            &self,
            other_position: Position<C>,
        ) -> Result<O, NearlySingularConversion> {
            O::orientation_between_positions(*self, other_position)
        }

        /// Gets the [`Orientation`](crate::orientation::Orientation) that points towards from this position from `other_position`
        ///
        /// # Example
        /// ```rust
        /// use leafwing_2d::position::Position;
        /// use leafwing_2d::orientation::{Direction, Orientation};
        ///
        /// let origin = Position::<f32>::default();
        /// let target = Position::new(0.0, 1.0);
        ///
        /// let direction: Direction = origin.orientation_from(target).expect("These positions are distinct.");
        /// direction.assert_approx_eq(Direction::SOUTH);
        /// ```
        #[inline]
        pub fn orientation_from<O: OrientationPositionInterop<C>>(
            &self,
            other_position: Position<C>,
        ) -> Result<O, NearlySingularConversion> {
            O::orientation_between_positions(other_position, *self)
        }
    }
}

mod coordinate {
    use crate::errors::FloatCoordinateConversionError;
    use std::{fmt::Debug, ops::*};

    /// A type that can be used as a coordinate type for [`Position`]
    ///
    /// Typically, you will want to use one of a few strategies for your [`Coordinate`] type:
    /// - if you just need simple continuous coordinates, use [`f32`]
    /// - if you're working with a grid-like position system, use one of the types provided in [`discrete_coordinates`]
    ///   - the [`DiscreteCoordinate`](discrete_coordinates::DiscreteCoordinate) trait provides other useful functionality for your game!
    /// - if you have unusual needs (such as extremely large worlds or tight memory constraints),
    /// implement [`Coordinate`] and optionally [`DiscreteCoordinate`](discrete_coordinates::DiscreteCoordinate) for your own type
    ///    - in this type, you can wrap your own [`f64`], [`u8`], fixed-point number type or so on.
    pub trait Coordinate:
        Copy
        + Debug
        + Default
        + Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + SubAssign
        + Mul<Output = Self>
        + MulAssign
        + Div<Output = Self>
        + DivAssign
        + Rem<Output = Self>
        + RemAssign
        + PartialOrd
        + Send
        + Sync
        + Into<f32>
        + 'static
    {
        /// Attempt to create a [`Coordinate`] from a `f32`, as might be returned by [`Transform`](bevy_transform::components::Transform)
        fn try_from_f32(float: f32) -> Result<Self, FloatCoordinateConversionError>;
    }

    impl<T> Coordinate for T
    where
        T: Copy
            + Debug
            + Default
            + Add<Output = Self>
            + AddAssign
            + Sub<Output = Self>
            + SubAssign
            + Mul<Output = Self>
            + MulAssign
            + Div<Output = Self>
            + DivAssign
            + Rem<Output = Self>
            + RemAssign
            + PartialOrd
            + Send
            + Sync
            + Into<f32>
            + TryFrom<f32>
            + 'static,
    {
        fn try_from_f32(float: f32) -> Result<Self, FloatCoordinateConversionError> {
            let result = float.try_into();

            match result {
                Ok(coordinate) => Ok(coordinate),
                Err(_) => Err(FloatCoordinateConversionError),
            }
        }
    }
}

mod positionlike {
    use super::{Coordinate, Position};
    use bevy_math::{Vec2, Vec3};
    use bevy_transform::components::{GlobalTransform, Transform};
    use core::fmt::Debug;

    /// A type that can be treated like a 2D (x,y) [`Position`]
    pub trait Positionlike: Sized + Copy + Debug + 'static {
        /// Converts this type into a [Vec2]
        fn into_vec2(self) -> Vec2;

        /// Asserts that `self` is approximately equal to `other`
        ///
        /// # Panics
        /// Panics if the distance between `self` and `other` is greater than 0.1.
        fn assert_approx_eq(self, other: impl Positionlike) {
            let self_vec2: Vec2 = self.into_vec2();
            let other_vec2: Vec2 = other.into_vec2();

            let distance = self_vec2.distance(other_vec2);
            assert!(
                distance <= 0.1,
                "{self:?} (converted to {self_vec2}) was {distance} away from {other:?} (converted to {other_vec2})."
            );
        }
    }

    impl<C: Coordinate> Positionlike for Position<C> {
        fn into_vec2(self) -> Vec2 {
            self.into()
        }
    }

    impl Positionlike for Transform {
        fn into_vec2(self) -> Vec2 {
            self.translation.truncate()
        }
    }

    impl Positionlike for GlobalTransform {
        fn into_vec2(self) -> Vec2 {
            self.translation.truncate()
        }
    }

    impl Positionlike for Vec2 {
        fn into_vec2(self) -> Vec2 {
            self
        }
    }

    impl Positionlike for Vec3 {
        fn into_vec2(self) -> Vec2 {
            self.truncate()
        }
    }
}

mod basic_operations {
    use super::{Coordinate, Position};
    use std::ops::*;

    impl<C: Coordinate> Add<Position<C>> for Position<C> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl<C: Coordinate> Sub<Position<C>> for Position<C> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl<C: Coordinate> Mul<C> for Position<C> {
        type Output = Position<C>;

        fn mul(self, rhs: C) -> Self::Output {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    impl<C: Coordinate> Div<C> for Position<C> {
        type Output = Position<C>;

        fn div(self, rhs: C) -> Self::Output {
            Self {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }

    impl<C: Coordinate> Rem<C> for Position<C> {
        type Output = Position<C>;

        fn rem(self, rhs: C) -> Self::Output {
            Self {
                x: self.x % rhs,
                y: self.y % rhs,
            }
        }
    }

    impl<C: Coordinate> Rem<Position<C>> for Position<C> {
        type Output = Position<C>;

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
    use crate::errors::{FloatCoordinateConversionError, NearlySingularConversion};
    use crate::orientation::{Direction, Rotation};
    use bevy_math::{Quat, Vec2, Vec3};
    use bevy_transform::components::{GlobalTransform, Transform};

    impl<C: Coordinate> TryFrom<Vec2> for Position<C> {
        type Error = FloatCoordinateConversionError;

        fn try_from(vec: Vec2) -> Result<Position<C>, FloatCoordinateConversionError> {
            let x = C::try_from_f32(vec.x)?;
            let y = C::try_from_f32(vec.y)?;

            Ok(Position { x, y })
        }
    }

    impl<C: Coordinate> From<Position<C>> for Vec2 {
        fn from(position: Position<C>) -> Vec2 {
            Vec2::new(position.x.into(), position.y.into())
        }
    }

    impl<C: Coordinate> From<Position<C>> for Vec3 {
        fn from(position: Position<C>) -> Vec3 {
            Vec3::new(position.x.into(), position.y.into(), 0.0)
        }
    }

    impl<C: Coordinate> TryFrom<Position<C>> for Direction {
        type Error = NearlySingularConversion;

        fn try_from(position: Position<C>) -> Result<Direction, NearlySingularConversion> {
            let vec2: Vec2 = position.into();

            vec2.try_into()
        }
    }

    impl<C: Coordinate> TryFrom<Position<C>> for Rotation {
        type Error = NearlySingularConversion;

        fn try_from(position: Position<C>) -> Result<Rotation, NearlySingularConversion> {
            let vec2: Vec2 = position.into();

            vec2.try_into()
        }
    }

    impl<C: Coordinate> TryFrom<Position<C>> for Quat {
        type Error = NearlySingularConversion;

        fn try_from(position: Position<C>) -> Result<Quat, NearlySingularConversion> {
            let direction: Direction = position.try_into()?;

            Ok(direction.into())
        }
    }

    impl<C: Coordinate> TryFrom<Vec3> for Position<C> {
        type Error = FloatCoordinateConversionError;

        fn try_from(vec: Vec3) -> Result<Position<C>, FloatCoordinateConversionError> {
            let x = C::try_from_f32(vec.x)?;
            let y = C::try_from_f32(vec.y)?;

            Ok(Position { x, y })
        }
    }

    impl<C: Coordinate> TryFrom<Transform> for Position<C> {
        type Error = FloatCoordinateConversionError;

        fn try_from(transform: Transform) -> Result<Position<C>, FloatCoordinateConversionError> {
            let x = C::try_from_f32(transform.translation.x)?;
            let y = C::try_from_f32(transform.translation.y)?;

            Ok(Position { x, y })
        }
    }

    impl<C: Coordinate> TryFrom<GlobalTransform> for Position<C> {
        type Error = FloatCoordinateConversionError;

        fn try_from(
            transform: GlobalTransform,
        ) -> Result<Position<C>, FloatCoordinateConversionError> {
            let x = C::try_from_f32(transform.translation.x)?;
            let y = C::try_from_f32(transform.translation.y)?;

            Ok(Position { x, y })
        }
    }
}

//! 2-dimensional coordinates

pub use coordinate::{Coordinate, TrivialCoordinate};
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
    use std::{fmt::Debug, ops::*};

    /// A type that can be used as a coordinate type for [`Position`]
    ///
    /// Typically, you will want to use one of a few strategies for your [`Coordinate`] type:
    /// - if you would like continuous coordinates for smooth movement, use one of the types provided in [`continuous`](crate::continuous)
    /// - if you're working with a grid-like position system, use one of the types provided in [`discrete`](crate::discrete)
    ///   - the [`DiscreteCoordinate`](crate::discrete::DiscreteCoordinate) trait provides other useful functionality for your game!
    /// - if you have unusual needs (such as extremely large worlds or tight memory constraints),
    /// implement [`Coordinate`] and optionally [`DiscreteCoordinate`](discrete_coordinates::DiscreteCoordinate) for your own type
    ///    - if you are simply wrapping a single value, strongly consider deriving [`TrivialCoordinate`] to reduce the required boilerplate
    ///    - in this type, you can wrap your own [`f64`], [`u8`], fixed-point number type or so on.
    ///
    /// # Conversion
    ///
    /// Each [`Coordinate`] type stores its own [`Coordinate::COORD_TO_TRANSFORM`] ratio,
    /// which is used to convert to and from [`Transform`](bevy_transform::components::Transform) correctly.
    /// When converting to and from [`Transform`](bevy_transform::components::Transform),
    /// [`GlobalTransform`](bevy_transform::components::GlobalTransform), [`Vec2`](bevy_math::Vec2) and [`Vec3`](bevy_math::Vec3),
    /// the values stored in your [`Coordinate`] component will be scaled appropriately to take advantage of the range
    /// of allowable transforms while ensuring that both conversions directions are infallible.
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
        + From<f32>
        + 'static
    {
        /// The ratio between 1 unit in this coordinate system to 1 unit of [`Transform.translation`](bevy_transform::components::Transform)
        const COORD_TO_TRANSFORM: f32;
    }

    /// A helper trait for [`Coordinate`] types that simply wrap a single number-like value
    ///
    /// Use `#derive(TrivialCoordinate)]` to implement all of the trivial trait bounds on [`Coordinate`] for you.
    /// All types that impl [`TrivialCoordinate`] + [`Into<f32>`] + [`TryFrom<f32>`]
    /// with a [`FloatCoordinateConversionError`] error type are automatically [`Coordinate`] types.
    ///
    /// # Example
    /// ```rust
    /// use leafwing_2d::position::{Coordinate, Trivial Coordinate};
    ///
    /// // This cause `TinyCoordinate` to automatically have the right trait impls
    /// // (e.g. Copy, Add, Debug and so on) due to blanket impls on `TrivialCoordinate`.
    /// #[derive(TrivialCoordinate`)]
    /// struct TinyCoordinate(u8);
    ///
    /// // We need a way to convert to and from Transform.translation
    /// // from our coordinate system.
    /// // In this example, we're choosing to scale our grid over a large fraction of the usable space of f32.
    /// const MAX_TRANSLATION: f32 = 1E+9;
    /// const SCALE_FACTOR: f32 = MAX_TRANSLATION / (u8::MAX as f32);
    ///
    /// // This automatically gets us the `Into<f32>` impl we need
    /// impl From<TinyCoordinate> for f32 {
    ///     fn from(coordinate: TinyCoordinate) -> f32 {
    ///         (coordinate as f32) * SCALE_FACTOR
    ///     }
    /// }
    ///
    /// // Be careful to ensure that these methods are the inverse of each other!
    /// impl TryFrom<f32> for TinyCoordinate {
    ///     type Error = FloatCoordinateConversionError;
    ///
    ///     fn from(float: f32) -> Result<TinyCoordinate, FloatCoordinateConversionError> {
    ///         if float <= MAX_TRANSLATION {
    ///            let integer = ((coordinate as f32) / SCALE_FACTOR).round() as u8;
    ///            Ok(TinyCoordinate(integer))
    ///         } else {
    ///             Err(FloatCoordinateConversionError(float))
    ///         }
    ///     }
    /// }
    ///
    /// // Congratulations: it's a `Coordinate`!
    /// let position: Position<TrivialCoordinate> = Position::new(0,0);
    /// ```
    pub trait TrivialCoordinate {
        type Wrapped;
        fn value(&self) -> Self::Wrapped;
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
    use crate::errors::NearlySingularConversion;
    use crate::orientation::{Direction, Rotation};
    use bevy_math::{Quat, Vec2, Vec3};
    use bevy_transform::components::{GlobalTransform, Transform};

    impl<C: Coordinate> From<Vec2> for Position<C> {
        fn from(vec: Vec2) -> Position<C> {
            let x = C::from(vec.x);
            let y = C::from(vec.y);

            Position { x, y }
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

    impl<C: Coordinate> From<Vec3> for Position<C> {
        fn from(vec: Vec3) -> Position<C> {
            let x = C::from(vec.x);
            let y = C::from(vec.y);

            Position { x, y }
        }
    }

    impl<C: Coordinate> From<Transform> for Position<C> {
        fn from(transform: Transform) -> Position<C> {
            let x = C::from(transform.translation.x);
            let y = C::from(transform.translation.y);

            Position { x, y }
        }
    }

    impl<C: Coordinate> From<GlobalTransform> for Position<C> {
        fn from(transform: GlobalTransform) -> Position<C> {
            let x = C::from(transform.translation.x);
            let y = C::from(transform.translation.y);

            Position { x, y }
        }
    }
}

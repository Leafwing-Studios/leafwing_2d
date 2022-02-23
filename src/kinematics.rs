//! Tools for working with [`Velocity`], [`Acceleration`] and their [`Rotation`]-based equivalents

use crate::position::Coordinate;
use bevy_ecs::component::Component;
use derive_more::{Add, AddAssign, Sub, SubAssign};

pub use kinematic_trait::Kinematic;

/// The rate of change of [`Position<C>`]
///
/// When used with [`linear_kinematics`](systems::linear_kinematics), the units are `C` per second
#[derive(Component, Default, Clone, Copy, Debug, PartialEq, Add, Sub, AddAssign, SubAssign)]
pub struct Velocity<C: Coordinate> {
    /// Velocity along the x-axis of a [`Position<C>`]
    pub x: C,
    /// Velocity along the y-axis of a [`Position<C>`]
    pub y: C,
}

/// The rate of change of [`Velocity<C>`]
///
/// When used with [`linear_kinematics`](systems::linear_kinematics), the units are `C` per second per second
#[derive(Component, Default, Clone, Copy, Debug, PartialEq, Add, Sub, AddAssign, SubAssign)]
pub struct Acceleration<C: Coordinate> {
    /// Acceleration along the x-axis of a [`Position<C>`]
    pub x: C,
    /// Acceleration along the y-axis of a [`Position<C>`]
    pub y: C,
}

/// The rate of change of [`Rotation`]
///
/// When used with [`angular_kinematics`](systems::angular_kinematics), the units are tenth of a degree per second
#[derive(Component, Default, Clone, Copy, Debug, PartialEq, Add, Sub, AddAssign, SubAssign)]
pub struct AngularVelocity {
    /// Tenth of a degree
    ///
    /// Positive is [`Clockwise`](crate::orientation::RotationDirection::Clockwise).
    /// Negative is [`CounterClockwise`](crate::orientation::RotationDirection::CounterClockwise).
    pub deci_degrees: isize,
}

/// The rate of change of [`AngularVelocity`]
///
/// When used with [`angular_kinematics`](systems::angular_kinematics), the units are tenth of a degree per second per second
#[derive(Component, Default, Clone, Copy, Debug, PartialEq, Add, Sub, AddAssign, SubAssign)]
pub struct AngularAcceleration {
    /// Tenth of a degree
    ///
    /// Positive is [`Clockwise`](crate::orientation::RotationDirection::Clockwise).
    /// Negative is [`CounterClockwise`](crate::orientation::RotationDirection::CounterClockwise).
    pub deci_degrees: isize,
}

mod kinematic_trait {
    use super::*;
    use crate::orientation::{Direction, RotationDirection};
    use crate::position::{Coordinate, Position};
    use core::ops::Mul;
    use std::time::Duration;

    /// A vector type that can be used as a [`Velocity`], [`Acceleration`] or so on
    pub trait Kinematic: Mul<f32> + Mul<Duration> {
        /// A measure of magnitude
        type M: PartialEq + PartialOrd + Default;
        /// A measure of direction
        type D: PartialEq;

        /// Creates a new vector, defined by its `magnitude` and `direction`
        fn new(magnitude: Self::M, direction: Self::D) -> Self;

        /// The strength (or length) of this vector
        fn magnitude(&self) -> Self::M;

        /// The squared strength (or length) of this vector
        fn magnitude_squared(&self) -> Self::M;

        /// The direction of this vector
        ///
        /// Returns `None` if the direction is neutral
        fn direction(&self) -> Option<Self::D>;
    }

    impl<C: Coordinate> Kinematic for Velocity<C> {
        type M = C;
        type D = Direction;

        fn new(magnitude: C, direction: Direction) -> Self {
            let magnitude: f32 = magnitude.into();
            let x = C::try_from_f32(magnitude * direction.unit_vector().y).unwrap();
            let y = C::try_from_f32(magnitude * direction.unit_vector().y).unwrap();

            Velocity { x, y }
        }

        fn magnitude(&self) -> C {
            let sum_of_squares: f32 = (self.x * self.x + self.y * self.y).into();
            C::try_from_f32(sum_of_squares.sqrt()).unwrap()
        }

        fn magnitude_squared(&self) -> C {
            self.x * self.x + self.y * self.y
        }

        fn direction(&self) -> Option<Direction> {
            if let Ok(direction) = Position::new(self.x, self.y).try_into() {
                Some(direction)
            } else {
                None
            }
        }
    }

    impl<C: Coordinate> Kinematic for Acceleration<C> {
        type M = C;
        type D = Direction;

        fn new(magnitude: C, direction: Direction) -> Self {
            let magnitude: f32 = magnitude.into();
            let x = C::try_from_f32(magnitude * direction.unit_vector().y).unwrap();
            let y = C::try_from_f32(magnitude * direction.unit_vector().y).unwrap();

            Acceleration { x, y }
        }

        fn magnitude(&self) -> C {
            let sum_of_squares: f32 = (self.x * self.x + self.y * self.y).into();
            C::try_from_f32(sum_of_squares.sqrt()).unwrap()
        }

        fn magnitude_squared(&self) -> C {
            self.x * self.x + self.y * self.y
        }

        fn direction(&self) -> Option<Direction> {
            if let Ok(direction) = Position::new(self.x, self.y).try_into() {
                Some(direction)
            } else {
                None
            }
        }
    }

    impl Kinematic for AngularVelocity {
        /// Tenths of a degree
        type M = isize;
        type D = RotationDirection;

        fn new(magnitude: isize, direction: RotationDirection) -> Self {
            AngularVelocity {
                deci_degrees: magnitude * direction.sign(),
            }
        }

        fn magnitude(&self) -> isize {
            self.deci_degrees.abs()
        }

        fn magnitude_squared(&self) -> isize {
            self.deci_degrees * self.deci_degrees
        }

        fn direction(&self) -> Option<RotationDirection> {
            let degrees = self.deci_degrees;
            match degrees {
                _neg if degrees < 0 => Some(RotationDirection::CounterClockwise),
                _zero if degrees == 0 => None,
                _pos if degrees > 0 => Some(RotationDirection::Clockwise),
                _ => unreachable!(),
            }
        }
    }

    impl Kinematic for AngularAcceleration {
        /// Tenths of a degree
        type M = isize;
        type D = RotationDirection;

        fn new(magnitude: isize, direction: RotationDirection) -> Self {
            AngularAcceleration {
                deci_degrees: magnitude * direction.sign(),
            }
        }

        fn magnitude(&self) -> isize {
            self.deci_degrees.abs()
        }

        fn magnitude_squared(&self) -> isize {
            self.deci_degrees * self.deci_degrees
        }

        fn direction(&self) -> Option<RotationDirection> {
            let degrees = self.deci_degrees;
            match degrees {
                _neg if degrees < 0 => Some(RotationDirection::CounterClockwise),
                _zero if degrees == 0 => None,
                _pos if degrees > 0 => Some(RotationDirection::Clockwise),
                _ => unreachable!(),
            }
        }
    }
}

/// Systems that apply kinematics in your `bevy` game.
///
/// These can be included as part of [`crate::plugin::TwoDPlugin`].
pub mod systems {
    use super::*;

    use crate::orientation::Rotation;
    use crate::position::Position;
    use bevy_core::Time;
    use bevy_ecs::prelude::*;

    /// Applies [`Acceleration`] and [`Velocity`] according to elapsed [`Time`]
    pub fn linear_kinematics<C: Coordinate>(
        time: Res<Time>,
        mut query: Query<(&mut Position<C>, &mut Velocity<C>, &Acceleration<C>)>,
    ) {
        let delta_time = time.delta();
        for (mut position, mut velocity, acceleration) in query.iter_mut() {
            *velocity += *acceleration * delta_time;
            *position += *velocity * delta_time;
        }
    }

    /// Applies [`AngularAcceleration`] and [`AngularVelocity`] according to elapsed [`Time`]
    pub fn angular_kinematics(
        time: Res<Time>,
        mut query: Query<(&mut Rotation, &mut AngularVelocity, &AngularAcceleration)>,
    ) {
        let delta_time = time.delta();
        for (mut position, mut velocity, acceleration) in query.iter_mut() {
            *velocity += *acceleration * delta_time;
            *position += *velocity * delta_time;
        }
    }
}

mod mul_f32 {
    use super::*;
    use crate::position::Coordinate;
    use std::ops::Mul;

    impl<C: Coordinate> Mul<f32> for Velocity<C> {
        type Output = Velocity<C>;

        fn mul(self, rhs: f32) -> Self::Output {
            let scale = C::try_from_f32(rhs).unwrap();

            Velocity {
                x: self.x * scale,
                y: self.y * scale,
            }
        }
    }

    impl<C: Coordinate> Mul<Velocity<C>> for f32 {
        type Output = Velocity<C>;

        fn mul(self, rhs: Velocity<C>) -> Self::Output {
            let scale = C::try_from_f32(self).unwrap();

            Velocity {
                x: rhs.x * scale,
                y: rhs.y * scale,
            }
        }
    }

    impl<C: Coordinate> Mul<f32> for Acceleration<C> {
        type Output = Acceleration<C>;

        fn mul(self, rhs: f32) -> Acceleration<C> {
            let scale = C::try_from_f32(rhs).unwrap();

            Acceleration {
                x: self.x * scale,
                y: self.y * scale,
            }
        }
    }

    impl<C: Coordinate> Mul<Acceleration<C>> for f32 {
        type Output = Acceleration<C>;

        fn mul(self, rhs: Acceleration<C>) -> Acceleration<C> {
            let scale = C::try_from_f32(self).unwrap();

            Acceleration {
                x: rhs.x * scale,
                y: rhs.y * scale,
            }
        }
    }

    impl Mul<f32> for AngularVelocity {
        type Output = AngularVelocity;

        fn mul(self, rhs: f32) -> Self::Output {
            let scale = rhs.round() as isize;

            Self {
                deci_degrees: self.deci_degrees * scale,
            }
        }
    }

    impl Mul<AngularVelocity> for f32 {
        type Output = AngularVelocity;

        fn mul(self, rhs: AngularVelocity) -> AngularVelocity {
            let scale = self.round() as isize;

            AngularVelocity {
                deci_degrees: rhs.deci_degrees * scale,
            }
        }
    }

    impl Mul<f32> for AngularAcceleration {
        type Output = AngularAcceleration;

        fn mul(self, rhs: f32) -> AngularAcceleration {
            let scale = rhs.round() as isize;

            Self {
                deci_degrees: self.deci_degrees * scale,
            }
        }
    }

    impl Mul<AngularAcceleration> for f32 {
        type Output = AngularAcceleration;

        fn mul(self, rhs: AngularAcceleration) -> AngularAcceleration {
            let scale = self.round() as isize;

            AngularAcceleration {
                deci_degrees: rhs.deci_degrees * scale,
            }
        }
    }
}

mod mul_duration {
    use super::*;
    use crate::orientation::Rotation;
    use crate::position::{Coordinate, Position};
    use std::ops::Mul;
    use std::time::Duration;

    impl<C: Coordinate> Mul<Duration> for Velocity<C> {
        type Output = Position<C>;

        fn mul(self, rhs: Duration) -> Position<C> {
            let scale = C::try_from_f32(rhs.as_secs_f32()).unwrap();

            Position {
                x: self.x * scale,
                y: self.y * scale,
            }
        }
    }

    impl<C: Coordinate> Mul<Velocity<C>> for Duration {
        type Output = Position<C>;

        fn mul(self, rhs: Velocity<C>) -> Position<C> {
            let scale = C::try_from_f32(self.as_secs_f32()).unwrap();

            Position {
                x: rhs.x * scale,
                y: rhs.y * scale,
            }
        }
    }

    impl<C: Coordinate> Mul<Duration> for Acceleration<C> {
        type Output = Velocity<C>;

        fn mul(self, rhs: Duration) -> Velocity<C> {
            let scale = C::try_from_f32(rhs.as_secs_f32()).unwrap();

            Velocity {
                x: self.x * scale,
                y: self.y * scale,
            }
        }
    }

    impl<C: Coordinate> Mul<Acceleration<C>> for Duration {
        type Output = Velocity<C>;

        fn mul(self, rhs: Acceleration<C>) -> Velocity<C> {
            let scale = C::try_from_f32(self.as_secs_f32()).unwrap();

            Velocity {
                x: rhs.x * scale,
                y: rhs.y * scale,
            }
        }
    }

    impl Mul<Duration> for AngularVelocity {
        type Output = Rotation;

        fn mul(self, rhs: Duration) -> Rotation {
            let scale = rhs.as_secs_f32() as isize;
            let new_deci_degrees = (self.deci_degrees * scale) % 3600;

            Rotation::new(new_deci_degrees as u16)
        }
    }

    impl Mul<AngularVelocity> for Duration {
        type Output = Rotation;

        fn mul(self, rhs: AngularVelocity) -> Rotation {
            let scale = self.as_secs_f32() as isize;
            let new_deci_degrees = (rhs.deci_degrees * scale) % 3600;

            Rotation::new(new_deci_degrees as u16)
        }
    }

    impl Mul<Duration> for AngularAcceleration {
        type Output = AngularVelocity;

        fn mul(self, rhs: Duration) -> AngularVelocity {
            let scale = rhs.as_secs_f32() as isize;

            AngularVelocity {
                deci_degrees: self.deci_degrees * scale,
            }
        }
    }

    impl Mul<AngularAcceleration> for Duration {
        type Output = AngularVelocity;

        fn mul(self, rhs: AngularAcceleration) -> AngularVelocity {
            let scale = self.as_secs_f32() as isize;

            AngularVelocity {
                deci_degrees: rhs.deci_degrees * scale,
            }
        }
    }
}

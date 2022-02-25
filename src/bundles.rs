//! [`Bundle`] Collections of components for creating

use crate::coordinate::Coordinate;
use crate::kinematics::{Acceleration, AngularAcceleration, AngularVelocity, Velocity};
use crate::orientation::{Direction, Rotation};
use crate::position::Position;
use bevy_ecs::prelude::Bundle;

/// A [`Bundle`] of components that store 2-dimensional information about position and orientation
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use leafwing_2d::plugin::TwoDBundle;
///
/// #[derive(Component, Default)]
/// struct Player;
///
/// #[derive(Bundle, Default)]
/// struct PlayerBundle {
///     player: Player,
///     #[bundle]
///     sprite: SpriteBundle,
///     #[bundle]
///     two_d: TwoDBundle<f32>,
/// }
/// ```
#[derive(Bundle, Clone, Debug, Default)]
pub struct TwoDBundle<C: Coordinate> {
    /// The 2-dimensional [`Position`] of the entity
    ///
    /// This is automatically converted into a [`Transform`]'s translation
    pub position: Position<C>,
    /// The rate of movement in `C` per second
    pub velocity: Velocity<C>,
    /// The rate at which velocity changes in `C` per second per second
    pub acceleration: Acceleration<C>,
    /// Which way the entity is facing, stored as an angle from due north
    pub rotation: Rotation,
    /// Which way the entity is facing, stored as a unit vector
    pub direction: Direction,
    /// The rate of rotation in deci-degrees per second
    pub angular_velocity: AngularVelocity,
    /// The rate at which angular velocity changes in deci-degrees per second per second
    pub angular_acceleration: AngularAcceleration,
}

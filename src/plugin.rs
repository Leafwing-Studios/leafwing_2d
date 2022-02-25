//! Tools for using two-dimensional coordinates within `bevy` games

use crate::continuous::F32;
use crate::coordinate::Coordinate;
use crate::kinematics::systems::{angular_kinematics, linear_kinematics};
use crate::orientation::{Direction, Rotation};
use crate::position::Position;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ShouldRun;
use bevy_ecs::system::Resource;
use bevy_math::Quat;
use bevy_transform::components::Transform;
use core::fmt::Debug;
use core::hash::Hash;
use core::marker::PhantomData;

/// Ensures that two-dimensional [`Position`], [`Direction`] and [`Rotation`] components are synchronized with the [`Transform`] equivalent
///
/// The type paramter `C` is the coordinate type used in [`Position`].
/// [`Transform`] can be modified directly, but if both the [`Transform`]
/// and its 2D analogue have been changed, the 2D version will take priority.
/// Similary, [`Rotation`] takes priority over [`Direction`].
///
/// System labels are stored in [`TwoDSystem`], which describes the working of this plugin in more depth.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use leafwing_2d::prelude::*;
/// use leafwing_2d::plugin::GameState;
/// use leafwing_2d::discrete::FlatHex;
/// use core::marker::PhantomData;
///
/// // This is a sensible starting point for a grid-based game
/// let app = App::new()
///     .add_state(GameState::Playing)
///     .add_plugin(TwoDPlugin {
///       kinematics: false,
///       kinematics_state: None,
///       stage: CoreStage::PostUpdate,
///       // Hexagons are the bestagons
///       coordinate_type: PhantomData::<FlatHex>::default(),
///      });
///
/// app.update();
/// ```
#[derive(Debug)]
pub struct TwoDPlugin<
    C: Coordinate,
    UserState: Resource + Eq + Debug + Clone + Hash,
    UserStage: StageLabel,
> {
    /// Should [`TwoDSystem::Kinematics] systems be enabled?
    ///
    /// Default: [`true`](bool)
    pub kinematics: bool,
    /// Kinematics are only computed during the provided state
    ///
    /// If `None`, kinematics are always run
    ///
    /// Default: [`None`]
    pub kinematics_state: Option<UserState>,
    /// Which stage should these systems run in?
    ///
    /// Default: [`CoreStage::PostUpdate`]
    pub stage: UserStage,
    /// What [`Coordinate`] should be used?
    ///
    /// Default: [`F32`]
    pub coordinate_type: PhantomData<C>,
}

impl Default for TwoDPlugin<F32, GameState, CoreStage> {
    fn default() -> Self {
        Self {
            kinematics: true,
            kinematics_state: None,
            stage: CoreStage::PostUpdate,
            coordinate_type: PhantomData::<F32>::default(),
        }
    }
}

/// Is the game paused?
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum GameState {
    /// The game is not paused
    Playing,
    /// The game is paused
    Paused,
}

/// [`SystemLabel`] for [`TwoDPlugin`]
///
/// These labels are executed in sequence.
#[derive(SystemLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TwoDSystem {
    /// Applies acceleration and velocity
    ///
    /// Contains [`linear_kinematics::<C>`] and [`angular_kinematics`].
    /// Disable these by setting the `kinematics` field of [`TwoDPlugin`].
    Kinematics,
    /// Synchronizes the [`Direction`] and [`Rotation`] of all entities
    ///
    /// If [`Direction`] and [`Rotation`] are desynced, whichever one was changed will be used and the other will be made consistent.
    /// If both were changed, [`Rotation`] will be prioritized
    ///
    /// Contains [`sync_direction_and_rotation`].
    SyncDirectionRotation,
    /// Synchronizes the [`Rotation`] and [`Position`] of each entity with its [`Transform`]
    ///
    /// Not all components are needed for this system to do its work.
    ///
    /// Contains [`sync_transform_with_2d`].
    SyncTransform,
}

impl<
        C: Coordinate,
        UserState: Resource + Eq + Debug + Clone + Hash,
        UserStage: StageLabel + Clone,
    > Plugin for TwoDPlugin<C, UserState, UserStage>
{
    fn build(&self, app: &mut App) {
        if self.kinematics {
            let kinematics_systems = SystemSet::new()
                .with_system(linear_kinematics::<C>)
                .with_system(angular_kinematics)
                .label(TwoDSystem::Kinematics)
                .before(TwoDSystem::SyncDirectionRotation);

            // If a state has been provided
            // Only run this plugin's systems in the state variant provided
            // Note that this does not perform the standard looping behavior
            // as otherwise we would be limited to the stage that state was added in T_T
            if let Some(desired_state_variant) = self.kinematics_state.clone() {
                // https://github.com/bevyengine/rfcs/pull/45 will make special-casing state support unnecessary

                // Captured the state variant we want our systems to run in in a run-criteria closure
                // The `SystemSet` methods take self by ownership, so we must store a new system set
                let kinematics_systems = kinematics_systems.with_run_criteria(
                    move |current_state: Res<State<UserState>>| {
                        if *current_state.current() == desired_state_variant {
                            ShouldRun::Yes
                        } else {
                            ShouldRun::No
                        }
                    },
                );

                app.add_system_set_to_stage(self.stage.clone(), kinematics_systems);
            } else {
                app.add_system_set_to_stage(self.stage.clone(), kinematics_systems);
            }
        }

        let sync_systems = SystemSet::new()
            .with_system(sync_direction_and_rotation.label(TwoDSystem::SyncDirectionRotation))
            .with_system(sync_transform_with_2d::<C>.label(TwoDSystem::SyncTransform));

        app.add_system_set_to_stage(self.stage.clone(), sync_systems);
    }
}

/// Synchronizes the [`Direction`] and [`Rotation`] of all entities
///
/// If [`Direction`] and [`Rotation`] are desynced, whichever one was changed will be used and the other will be made consistent.
/// If both were changed, [`Rotation`] will be prioritized
pub fn sync_direction_and_rotation(mut query: Query<(&mut Direction, &mut Rotation)>) {
    for (mut direction, mut rotation) in query.iter_mut() {
        if rotation.is_changed() {
            let new_direction: Direction = (*rotation).into();
            // These checks are required to avoid triggering change detection pointlessly,
            // which would create an infinite ping-pong effect
            if *direction != new_direction {
                *direction = new_direction;
            }
        } else if direction.is_changed() {
            let new_rotation = (*direction).into();
            if *rotation != new_rotation {
                *rotation = new_rotation;
            }
        }
    }
}

/// Synchronizes the [`Rotation`], [`Direction`] and [`Position`] of each entity with its [`Transform`] and vice versa
///
/// [`Transform`] can be modified directly, but if both the [`Transform`]
/// and its 2D analogue have been changed, the 2D version will take priority.
///
/// z-values of the [`Transform`] translation will not be modified.
/// Any off-axis rotation of the [`Transform`]'s rotation quaternion will be lost.
// FIXME: also sync `Scale`.
pub fn sync_transform_with_2d<C: Coordinate>(
    mut query: Query<
        (
            &mut Transform,
            Option<&mut Rotation>,
            Option<&mut Direction>,
            Option<&mut Position<C>>,
        ),
        Or<(With<Rotation>, With<Position<C>>)>,
    >,
) {
    for (mut transform, maybe_rotation, maybe_direction, maybe_position) in query.iter_mut() {
        // Synchronize Rotation with Transform
        if let Some(mut rotation) = maybe_rotation {
            if rotation.is_changed() {
                let new_quat: Quat = (*rotation).into();
                if transform.rotation != new_quat {
                    transform.rotation = new_quat;
                }
            } else if transform.is_changed() {
                if let Ok(new_rotation) = transform.rotation.try_into() {
                    if *rotation != new_rotation {
                        *rotation = new_rotation;
                    }
                }
            }
        }

        // Synchronize Direction with Transform
        if let Some(mut direction) = maybe_direction {
            if direction.is_changed() {
                let new_quat = (*direction).into();
                if transform.rotation != new_quat {
                    transform.rotation = new_quat;
                }
            } else if transform.is_changed() && *direction != transform.rotation.into() {
                *direction = transform.rotation.into();
            }
        }

        // Synchronize Position with Transform
        if let Some(mut position) = maybe_position {
            if position.is_changed() {
                let new_x: f32 = position.x.into();
                if transform.translation.x != new_x {
                    transform.translation.x = new_x;
                }

                let new_y: f32 = position.y.into();
                if transform.translation.y != new_y {
                    transform.translation.y = new_y;
                }
            } else if transform.is_changed() {
                let new_x = C::from(transform.translation.x);
                if position.x != new_x {
                    position.x = new_x;
                }

                let new_y = C::from(transform.translation.y);
                if position.y != new_y {
                    position.y = new_y;
                }
            }
        }
    }
}

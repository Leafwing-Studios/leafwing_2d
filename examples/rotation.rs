//! Demonstrates how to handle rotations using the `leafwing_2d::Orientation` methods
//!
//! This is a direct conversion of the [`bevy` example of the same name](https://github.com/bevyengine/bevy/blob/main/examples/2d/rotation.rs)
//! Used under the MIT License courtesy of Bevy contributors

use bevy::{core::FixedTimestep, math::const_vec2, prelude::*};
use leafwing_2d::bounding::AxisAlignedBoundingBox;
use leafwing_2d::prelude::*;
// This is part of the prelude of leafwing_2d, but clashes with the `bevy_ui` version :/
use leafwing_2d::orientation::Direction;

const TIME_STEP: f32 = 1.0 / 60.0;
const PLAY_AREA: AxisAlignedBoundingBox<f32> = AxisAlignedBoundingBox {
    low_x: -600.0,
    low_y: -320.0,
    high_x: 600.0,
    high_y: 320.0,
};

const BOUNDS: Vec2 = const_vec2!([1200.0, 640.0]);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TwoDimPlugin::<f32>::default())
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system)
                .with_system(snap_to_player_system)
                .with_system(rotate_to_player_system),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

/// player component
#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}

/// snap to player ship behavior
#[derive(Component)]
struct SnapToPlayer;

/// rotate to face player ship behavior
#[derive(Component)]
struct RotateToPlayer {
    /// rotation speed in radians per second
    rotation_speed: f32,
}

/// Add the game's entities to our world and creates an orthographic camera for 2D rendering.
///
/// The Bevy coordinate system is the same for 2D and 3D, in terms of 2D this means that:
///
/// * X axis goes from left to right (+X points right)
/// * Y axis goes from bottom to top (+Y point up)
/// * Z axis goes from far to near (+Z points towards you, out of the screen)
///
/// The origin is at the center of the screen.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("textures/simplespace/ship_C.png");
    let enemy_a_handle = asset_server.load("textures/simplespace/enemy_A.png");
    let enemy_b_handle = asset_server.load("textures/simplespace/enemy_B.png");

    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let horizontal_margin = BOUNDS.x / 4.0;
    let vertical_margin = BOUNDS.y / 4.0;

    // player controlled ship
    commands
        .spawn_bundle(SpriteBundle {
            texture: ship_handle,
            ..Default::default()
        })
        .insert(Position::<f32>::default())
        .insert(Rotation::default())
        .insert(Player {
            movement_speed: 500.0,                  // metres per second
            rotation_speed: f32::to_radians(360.0), // degrees per second
        });

    // enemy that snaps to face the player spawns on the bottom and left
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_a_handle.clone(),
            ..Default::default()
        })
        .insert(Position::<f32>::default())
        .insert(Rotation::default())
        .insert(SnapToPlayer);
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_a_handle,
            ..Default::default()
        })
        .insert(Position::<f32>::new(0.0, -vertical_margin))
        .insert(Rotation::default())
        .insert(SnapToPlayer);

    // enemy that rotates to face the player enemy spawns on the top and right
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_b_handle.clone(),
            ..Default::default()
        })
        .insert(Position::<f32>::new(horizontal_margin, 0.0))
        .insert(Rotation::default())
        .insert(RotateToPlayer {
            rotation_speed: f32::to_radians(45.0), // degrees per second
        });
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_b_handle,
            ..Default::default()
        })
        .insert(Position::<f32>::new(0.0, vertical_margin))
        .insert(Rotation::default())
        .insert(RotateToPlayer {
            rotation_speed: f32::to_radians(90.0), // degrees per second
        });
}

/// Demonstrates applying rotation and movement based on keyboard input.
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Position<f32>, &mut Rotation)>,
) {
    let (ship, mut position, mut rotation) = query.single_mut();

    // Apply rotation before movement to ensure that we are moving in the most recent direction
    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    // Left rotates you counterclockwise
    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor -= 1.0;
    }

    // Right rotates you clockwise
    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor += 1.0;
    }

    // create the change in rotation around the Z axis (perpendicular to the 2D plane of the screen)
    let rotation_delta = Rotation::from_radians(rotation_factor * ship.rotation_speed * TIME_STEP);
    // update the ship rotation with our rotation delta
    *rotation += rotation_delta;

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * ship.movement_speed * TIME_STEP;

    // Determine which direction to move in based on our rotation
    let direction: Direction = (*rotation).into();

    // create the change in translation using the new movement direction and distance
    let translation_delta = direction * movement_distance;
    // update the ship translation with our new translation delta
    *position += translation_delta
        .try_into()
        .expect("This conversion is infallible for `Position<f32>`.");

    // bound the ship within the invisible level bounds
    *position = PLAY_AREA.clamp(*position);
}

/// Demonstrates snapping the enemy ship to face the player ship immediately.
fn snap_to_player_system(
    mut query: Query<(&mut Rotation, &Position<f32>), (With<SnapToPlayer>, Without<Player>)>,
    player_query: Query<&Position<f32>, With<Player>>,
) {
    let &player_position = player_query.single();

    for (mut enemy_rotation, &enemy_position) in query.iter_mut() {
        // rotate to face the player
        enemy_rotation.rotate_towards_position(enemy_position, player_position, None);
    }
}

/// Demonstrates rotating an enemy ship to face the player ship at a given rotation speed.
///
/// This system simply uses the `rotation_to` and `rotate_towards` methods to perform the required computations.
fn rotate_to_player_system(
    mut query: Query<(&RotateToPlayer, &mut Rotation, &Position<f32>), Without<Player>>,
    player_query: Query<&Position<f32>, With<Player>>,
) {
    let &player_position = player_query.single();

    for (config, mut enemy_rotation, enemy_position) in query.iter_mut() {
        // compute the maximum amount that this entity is allowed to turn in this time step
        let max_rotation = Rotation::from_radians(config.rotation_speed * TIME_STEP);

        // rotate towards the player by up to the max_rotation
        enemy_rotation.rotate_towards_position(
            *enemy_position,
            player_position,
            Some(max_rotation),
        );
    }
}

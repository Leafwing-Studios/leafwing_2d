//! A minimal example of how to use `leafwing_2d`

use bevy::prelude::*;
use leafwing_2d::prelude::*;
// On 0.6, there is a namespace clash with `bevy_ui::FlexDirection`
use leafwing_2d::orientation::Direction;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // `f32` is our `Coordinate` unit for this example
        .add_plugin(TwoDPlugin::default())
        .add_startup_system(setup)
        // We can work directly with `Position`, `Direction` and `Rotation`
        .add_system(rotate_player)
        // Convenient methods for converting to-and-from screen-space are included
        .add_system(move_towards_click)
        // Or, we can use the included kinematics to work in terms of velocity and acceleration
        .add_system(accelerate_player)
        // Use an AABB to ensure the player doesn't go out of bounds
        .add_system(bound_player)
        .run();
}

#[derive(Component, Default)]
struct Player;

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    #[bundle]
    sprite: SpriteBundle,
    #[bundle]
    two_d: TwoDBundle<F32>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(PlayerBundle {
        sprite: SpriteBundle {
            texture: asset_server.load("textures/simplespace/ship_C.png"),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn rotate_player(mut query: Query<&mut Rotation, With<Player>>, input: Res<Input<KeyCode>>) {
    let mut rotation = query.single_mut();

    if input.pressed(KeyCode::Left) {
        *rotation -= Rotation::from_degrees(5.0);
    }

    if input.pressed(KeyCode::Right) {
        *rotation += Rotation::from_degrees(5.0);
    }
}

fn move_towards_click() {}

fn accelerate_player(
    mut query: Query<(&Direction, &mut Velocity<f32>), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let (&direction, mut velocity) = query.single_mut();

    if input.pressed(KeyCode::Up) {
        *velocity += Velocity::new(10., direction);
    }
}

fn bound_player(query: Query<&mut Transform, With<Player>>, windows: Res<Windows>) {
    // Notice that we can set Transform directly, and the 2D versions are synced
    let player_transform = query.single_mut();

    let window = windows.get_primary().unwrap();
    let aabb =
        AxisAlignedBoundingBox::from_size(Position::default(), window.width(), window.height());

    *player_position = aabb.clamp(player_position);
}

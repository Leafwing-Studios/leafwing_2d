//! A minimal example of how to use `leafwing_2d`

use bevy::prelude::*;
use leafwing_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TwoDPlugin::<f32>::default())
        .add_startup_system(setup)
        .add_system(rotate_player)
        .add_system(set_player_direction)
        .add_system(move_player)
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
    two_d: TwoDBundle<f32>,
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

fn rotate_player() {}

fn set_player_direction() {}

fn move_player() {}

#![warn(
clippy::all,
// clippy::restriction,
clippy::pedantic,
clippy::nursery,
// clippy::cargo
)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::future_not_send)]
#![allow(clippy::fallible_impl_from)]
#![allow(clippy::single_match)]
mod camera;
mod movement;
mod player;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use camera::CameraPlugin;
use movement::Directions;
use movement::MovementPlugin;
use player::PlayerPlugin;

const WINDOW_WIDTH: f32 = 496.0;
const WINDOW_HEIGHT: f32 = 672.0;
const CELL_SIZE: f32 = 24.;

const STARTING_POSITION_X: f32 = 2.;
const STARTING_POSITION_Y: f32 = 1.;
const STARTING_DIRECTION: Directions = Directions::Down;
const PLAYER_VELOCITY: f32 = 8.;

#[derive(Resource)]
pub struct GameState {
    pub is_debug: bool,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(
                    // This sets image filtering to nearest
                    // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
                    // by linear filtering.
                    ImagePlugin::default_nearest(),
                )
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // provide the ID selector string here
                        canvas: Some("#game-canvas".into()),
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        // ... any other window properties ...
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .insert_resource(GameState { is_debug: false })
        .run();
}

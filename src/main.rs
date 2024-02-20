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
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::needless_pass_by_value)]
mod board;
mod camera;
mod collision;
mod debug;
mod game_assets;
mod input;
mod movement;
mod pickup;
mod player;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use board::BoardPlugin;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use debug::DebugGizmos;
use debug::DebugPlugin;
use game_assets::AssetLoaderPlugin;
use input::InputPlugin;
use movement::Directions;
use movement::MovementPlugin;
use player::PlayerPlugin;

const WINDOW_WIDTH: f32 = 448.0;
const WINDOW_HEIGHT: f32 = 496.0;
const CELL_SIZE: f32 = 16.;

const STARTING_POSITION_X: f32 = 14.;
const STARTING_POSITION_Y: f32 = 23.;
const STARTING_DIRECTION: Directions = Directions::Down;
const PLAYER_VELOCITY: f32 = 8.;

const POWERPILL_SCORE: usize = 50;
const DOT_SCORE: usize = 10;
const PICKUP_RANGE: f32 = 0.5;

#[derive(Default, Resource)]
pub struct GameState {
    pub score: usize,
    pub show_grid: bool,
    pub is_debug: bool,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
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
        .init_gizmo_group::<DebugGizmos>()
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(BoardPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(DebugPlugin)
        .insert_resource(GameState::default())
        .run();
}

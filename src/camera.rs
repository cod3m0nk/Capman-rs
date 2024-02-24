use crate::{BOARD_HEIGHT, BOARD_WIDTH, CELL_SIZE, UI_HEIGHT, WINDOW_HEIGHT};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut my_2d_camera_bundle = Camera2dBundle::default();

    // Set the scaling mode for the camera projection so the board always fills the screen
    // vertically
    my_2d_camera_bundle.projection.scaling_mode =
        ScalingMode::FixedVertical(WINDOW_HEIGHT + UI_HEIGHT);

    my_2d_camera_bundle.projection.viewport_origin = Vec2 { x: 0.5, y: 0.5 };

    // Calculate the X position of the camera by dividing half of the board width by the cell size
    let camera_x = CELL_SIZE * BOARD_WIDTH / 2.;

    // Calculate the Y position of the camera by adding half of the board height to UI_HEIGHT,
    // and then dividing by the cell size. Negative sign is used because the y-axis in bevy is
    // flipped compared to traditional graphics systems
    let camera_y = (-CELL_SIZE).mul_add(BOARD_HEIGHT, UI_HEIGHT) / 2.;

    my_2d_camera_bundle.transform = Transform::from_xyz(camera_x, camera_y, 0.);

    commands.spawn(my_2d_camera_bundle);
}

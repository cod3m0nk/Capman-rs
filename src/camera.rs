use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    use bevy::render::camera::ScalingMode;

    let mut my_2d_camera_bundle = Camera2dBundle::default();
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: WINDOW_WIDTH,
        max_height: WINDOW_HEIGHT,
    };
    my_2d_camera_bundle.projection.viewport_origin = Vec2 { x: 0., y: 1. };
    commands.spawn(my_2d_camera_bundle);
}

use crate::{board::Board, state::GameGlobals, CELL_SIZE};
use bevy::prelude::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_debug)
            .add_systems(Startup, setup);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct DebugGizmos;

fn setup(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DebugGizmos>();
    config.line_width = 0.1;
}

fn draw_debug(gizmos: Gizmos<DebugGizmos>, board: Res<Board>, game_globals: Res<GameGlobals>) {
    if game_globals.show_grid {
        draw_grid(board, gizmos);
    }
}

fn draw_grid(board: Res<'_, Board>, mut gizmos: Gizmos<'_, '_, DebugGizmos>) {
    let (rows, columns) = board.get_dimensions();
    for row in 1..rows {
        gizmos.line_2d(
            Vec2::new(0., row as f32 * -CELL_SIZE),
            Vec2::new(columns as f32 * CELL_SIZE, row as f32 * -CELL_SIZE),
            Color::GREEN,
        );
    }
    for column in 1..columns {
        gizmos.line_2d(
            Vec2::new(column as f32 * CELL_SIZE, 0.),
            Vec2::new(column as f32 * CELL_SIZE, rows as f32 * -CELL_SIZE),
            Color::GREEN,
        );
    }
}

use crate::input::GameStateChangeEvent;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_systems(
            Update,
            game_state_input_events.run_if(on_event::<GameStateChangeEvent>()),
        );
    }
}

#[derive(Default, Resource)]
pub struct GameGlobals {
    pub score: usize,
    pub show_grid: bool,
    pub is_debug: bool,
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if *state.get() == GameState::Running {
        next_state.set(GameState::Paused);
    } else {
        next_state.set(GameState::Running);
    }
}

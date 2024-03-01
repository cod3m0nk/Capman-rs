use crate::INTIAL_LIVES;
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
        app.init_state::<GameState>()
            .insert_resource(GameGlobals {
                lives: INTIAL_LIVES,
                ..default()
            })
            .add_systems(Update, process_game_events.run_if(on_event::<GameEvent>()));
    }
}

#[derive(Event)]
pub enum GameEvent {
    TogglePause,
    PlayerDies,
}

#[derive(Default, Resource)]
pub struct GameGlobals {
    pub score: usize,
    pub show_grid: bool,
    pub is_debug: bool,
    pub lives: isize,
}

fn process_game_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    mut event_reader: EventReader<GameEvent>,
    mut globals: ResMut<GameGlobals>,
) {
    for event in event_reader.read() {
        match event {
            GameEvent::TogglePause => {
                if *state.get() == GameState::Running {
                    next_state.set(GameState::Paused);
                } else {
                    next_state.set(GameState::Running);
                }
            }
            GameEvent::PlayerDies => {
                globals.lives -= 1;
            }
        }
    }
}

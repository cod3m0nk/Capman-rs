use crate::state::GameGlobals;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, read_inputs)
            .add_event::<InputDirectionEvent>()
            .add_event::<GameStateChangeEvent>();
    }
}

#[derive(Event)]
pub enum InputDirectionEvent {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Event)]
pub enum GameStateChangeEvent {
    Toggle,
}

fn read_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction_event_writer: EventWriter<InputDirectionEvent>,
    mut game_state_event_writer: EventWriter<GameStateChangeEvent>,
    mut game_globals: ResMut<GameGlobals>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft)
    {
        direction_event_writer.send(InputDirectionEvent::Left);
    } else if keyboard_input.just_pressed(KeyCode::KeyD)
        || keyboard_input.just_pressed(KeyCode::ArrowRight)
    {
        direction_event_writer.send(InputDirectionEvent::Right);
    } else if keyboard_input.just_pressed(KeyCode::KeyW)
        || keyboard_input.just_pressed(KeyCode::ArrowUp)
    {
        direction_event_writer.send(InputDirectionEvent::Up);
    } else if keyboard_input.just_pressed(KeyCode::KeyS)
        || keyboard_input.just_pressed(KeyCode::ArrowDown)
    {
        direction_event_writer.send(InputDirectionEvent::Down);
    } else if keyboard_input.just_pressed(KeyCode::KeyG) {
        game_globals.show_grid ^= true;
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        game_state_event_writer.send(GameStateChangeEvent::Toggle);
    }
}

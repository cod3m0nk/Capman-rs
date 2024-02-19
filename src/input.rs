use bevy::prelude::*;

pub struct InputPlugin;

#[derive(Event)]
pub enum InputDirectionEvent {
    Up,
    Down,
    Left,
    Right,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, read_inputs)
            .add_event::<InputDirectionEvent>();
    }
}

fn read_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction_event_writer: EventWriter<InputDirectionEvent>,
) {
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_event_writer.send(InputDirectionEvent::Left);
    } else if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_event_writer.send(InputDirectionEvent::Right);
    } else if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction_event_writer.send(InputDirectionEvent::Up);
    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction_event_writer.send(InputDirectionEvent::Down);
    }
}

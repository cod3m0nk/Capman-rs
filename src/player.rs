use crate::input::InputDirectionEvent;
use crate::movement::Direction;
use crate::movement::Directions;
use crate::movement::MovingObjectBundle;
use crate::movement::Position;
use crate::movement::Velocity;
use crate::PLAYER_VELOCITY;
use crate::STARTING_DIRECTION;
use crate::STARTING_POSITION_X;
use crate::STARTING_POSITION_Y;
use bevy::prelude::*;

#[derive(Bundle)]
struct PlayerBundle {
    moving_object: MovingObjectBundle,
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement_control);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Position::new(STARTING_POSITION_X, STARTING_POSITION_Y);
    let transform = Transform::from(&position);
    commands.spawn((
        PlayerBundle {
            moving_object: MovingObjectBundle {
                position,
                velocity: Velocity::new(PLAYER_VELOCITY),
                dir: Direction::new(STARTING_DIRECTION, STARTING_DIRECTION),
            },
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/capman.png"),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::Center,
                    rect: Some(Rect::new(24., 0., 48., 24.)),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            },
        },
        Player,
    ));
}

fn player_movement_control(
    mut query: Query<&mut Direction, With<Player>>,
    mut input_event_reader: EventReader<InputDirectionEvent>,
) {
    let mut direction = query.single_mut();
    for input_event in input_event_reader.read() {
        match input_event {
            InputDirectionEvent::Up => direction.current = Directions::Up,
            InputDirectionEvent::Down => direction.current = Directions::Down,
            InputDirectionEvent::Left => direction.current = Directions::Left,
            InputDirectionEvent::Right => direction.current = Directions::Right,
        }
    }
}

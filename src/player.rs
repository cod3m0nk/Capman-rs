use crate::board::CellType;
use crate::game_assets::GameAssets;
use crate::game_assets::GameAssetsLoader;
use crate::input::InputDirectionEvent;
use crate::movement::Direction;
use crate::movement::Directions;
use crate::movement::MovableObject;
use crate::movement::MovingObjectBundle;
use crate::movement::Position;
use crate::movement::Velocity;
use crate::spritesheet::AnimatedSpriteBundle;
use crate::spritesheet::AnimationStrategy;
use crate::spritesheet::SpriteSheetAnimator;
use crate::PLAYER_VELOCITY;
use crate::STARTING_DIRECTION;
use crate::STARTING_POSITION_X;
use crate::STARTING_POSITION_Y;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement_control)
            .init_state::<PlayerState>();
    }
}

#[derive(Component)]
pub struct Player;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerState {
    #[default]
    Moving,
    Idle,
}

impl MovableObject for Player {
    fn update_direction(&self, pos: &Position, dir: &mut Direction, board: &crate::board::Board) {
        if dir.current == dir.next {
            return;
        }

        let next = pos.get_target_cell(dir.next);
        if !matches!(board.get_cell(&next), CellType::Wall(_)) {
            dir.current = dir.next;
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    game_assets: Res<GameAssetsLoader>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let position = Position::new(STARTING_POSITION_X, STARTING_POSITION_Y);
    let transform = Transform::from(&position);
    let layout = TextureAtlasLayout::from_grid(Vec2::new(24., 24.), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        MovingObjectBundle {
            position,
            velocity: Velocity::new(PLAYER_VELOCITY),
            dir: Direction::new(STARTING_DIRECTION, STARTING_DIRECTION),
        },
        AnimatedSpriteBundle {
            sprite_sheet_animator: SpriteSheetAnimator {
                start: 0,
                end: 2,
                frame_rate: 10.,
                strategy: AnimationStrategy::PingPong,
                texture_atlas_layout: texture_atlas_layout.clone(),
                ..Default::default()
            },
            spritesheet_bundle: SpriteSheetBundle {
                texture: game_assets.get(GameAssets::Player),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::Center,
                    ..Default::default()
                },
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
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
            InputDirectionEvent::Up => direction.next = Directions::Up,
            InputDirectionEvent::Down => direction.next = Directions::Down,
            InputDirectionEvent::Left => direction.next = Directions::Left,
            InputDirectionEvent::Right => direction.next = Directions::Right,
        }

        // If the next direction is opposite to the current one update the
        // direction immediately
        match (direction.next, direction.current) {
            (Directions::Up, Directions::Down)
            | (Directions::Down, Directions::Up)
            | (Directions::Left, Directions::Right)
            | (Directions::Right, Directions::Left) => {
                direction.current = direction.next;
            }
            _ => (),
        }
    }
}

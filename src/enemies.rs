use crate::{
    board::{Board, CellType},
    game_assets::{GameAssets, GameAssetsLoader},
    movement::{Direction, MovableObject, MovingObjectBundle, Position, Velocity},
    spritesheet::{AnimatedSpriteBundle, AnimationStrategy, SpriteSheetAnimator},
    PLAYER_VELOCITY, STARTING_DIRECTION,
};
use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
    }
}

#[derive(Component, Clone, Copy)]
pub struct Enemy {
    start_position: Vec2,
    enemy_ai: EnemyAI,
}

impl Enemy {
    pub const fn new(start_position: Vec2, _enemy_ai: EnemyAI) -> Self {
        Self {
            start_position,
            enemy_ai: EnemyAI::Random,
        }
    }
}

#[derive(Clone, Copy)]
pub enum EnemyAI {
    Random,
}

impl MovableObject for Enemy {
    fn update_direction(&self, pos: &Position, dir: &mut Direction, board: &Board) {
        dir.current = match self.enemy_ai {
            EnemyAI::Random => {
                let mut directions = board.get_neighbours(pos.x, pos.y);
                directions
                    .retain(|(_, cell)| !matches!(cell, CellType::Wall(_) | CellType::Outside));
                directions.retain(|(new_dir, _)| !dir.is_opposite(*new_dir));
                let mut rng = thread_rng();
                directions.choose(&mut rng).unwrap().0
            }
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    game_assets: Res<GameAssetsLoader>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    board: Res<Board>,
) {
    for enemy in board.get_enemies() {
        let position = Position::new(enemy.start_position.x, enemy.start_position.y);
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
                    texture: game_assets.get(GameAssets::Blinkus),
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
            *enemy,
        ));
    }
}

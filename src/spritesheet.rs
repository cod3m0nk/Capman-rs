use crate::{
    movement::Direction,
    player::{Player, PlayerState},
    state::GameState,
};
use bevy::prelude::*;

pub struct AnimatedSpritePlugin;
impl Plugin for AnimatedSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            update_player
                .run_if(in_state(PlayerState::Moving).and_then(in_state(GameState::Running))),
        );
    }
}

#[derive(Bundle)]
pub struct AnimatedSpriteBundle {
    pub sprite_sheet_animator: SpriteSheetAnimator,
    pub spritesheet_bundle: SpriteSheetBundle,
}

#[derive(Component, Default)]
pub struct SpriteSheetAnimator {
    pub start: usize,
    pub end: usize,
    pub frame_rate: f32,
    pub strategy: AnimationStrategy,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    pub time: f32,
}

impl SpriteSheetAnimator {
    pub fn update_index(&mut self, texture_atlas: &mut TextureAtlas, delta: f32) {
        self.time += delta;
        let time = (self.time * self.frame_rate) as usize;
        let animation_length = 1 + self.end - self.start;

        let index = match self.strategy {
            AnimationStrategy::PingPong => {
                if (time / animation_length % 2) == 0 {
                    time % animation_length
                } else {
                    animation_length - 1 - (time % animation_length)
                }
            }
            AnimationStrategy::Loop => {
                (self.time * self.frame_rate) as usize % (1 + self.end - self.start)
            }
        };
        texture_atlas.index = index;
    }
}

#[derive(Default)]
pub enum AnimationStrategy {
    #[default]
    Loop,
    PingPong,
}

fn update_player(
    mut query: Query<
        (
            &Direction,
            &mut Transform,
            &mut Sprite,
            &mut TextureAtlas,
            &mut SpriteSheetAnimator,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (dir, mut transform, mut sprite, mut atlas, mut animator) = query.single_mut();
    let (rot, flip) = match dir.current {
        crate::movement::Directions::Up => (270f32.to_radians(), false),
        crate::movement::Directions::Down => (90f32.to_radians(), false),
        crate::movement::Directions::Left => (0f32.to_radians(), false),
        crate::movement::Directions::Right => (0f32.to_radians(), true),
    };
    transform.rotation = Quat::from_rotation_z(rot);
    sprite.flip_x = flip;

    animator.update_index(&mut atlas, time.delta_seconds());
}

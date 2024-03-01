use crate::{
    enemies::Enemy,
    movement::Position,
    pickup::{Dot, Pickup, PowerPill},
    player::Player,
    state::GameEvent,
    state::{GameGlobals, GameState},
};
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            player_collision_detection.run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Component)]
pub struct Collider {
    pub distance: f32,
}

impl Collider {
    pub const fn new(distance: f32) -> Self {
        Self { distance }
    }
}

fn player_collision_detection(
    mut commnands: Commands,
    query_powerpill: Query<(Entity, &Position, &Collider, &Pickup), With<PowerPill>>,
    query_dot: Query<(Entity, &Position, &Collider, &Pickup), With<Dot>>,
    query_enemy: Query<(&Position, &Collider), With<Enemy>>,
    player_query: Query<&Position, With<Player>>,
    mut game_globals: ResMut<GameGlobals>,
    mut event_writer: EventWriter<GameEvent>,
) {
    let player_position = player_query.get_single().unwrap();
    for (entity, position, collider, pickup) in query_powerpill.iter() {
        if player_position.get_distance(position) < collider.distance {
            game_globals.score += pickup.get_value();
            commnands.entity(entity).despawn_recursive();
        }
    }
    for (entity, position, collider, pickup) in query_dot.iter() {
        if player_position.get_distance(position) < collider.distance {
            game_globals.score += pickup.get_value();
            commnands.entity(entity).despawn_recursive();
        }
    }
    for (position, collider) in query_enemy.iter() {
        if player_position.get_distance(position) < collider.distance {
            event_writer.send(GameEvent::PlayerDies);
        }
    }
}

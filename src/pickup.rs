use bevy::{
    ecs::{bundle::Bundle, component::Component},
    sprite::SpriteBundle,
};

use crate::movement::Position;

#[derive(Bundle)]
pub struct PickupBundle {
    pub pickup: Pickup,
    pub sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Pickup {
    position: Position,
    radius: f32,
}

impl Pickup {
    pub fn new(position: Position, radius: f32) -> Self {
        Self { position, radius }
    }
}

#[derive(Component)]
pub struct Dot;

#[derive(Component)]
pub struct PowerPill;

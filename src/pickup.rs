use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Pickup {
    value: usize,
}

impl Pickup {
    pub const fn new(value: usize) -> Self {
        Self { value }
    }
}

#[derive(Component)]
pub struct Dot;

#[derive(Component)]
pub struct PowerPill;

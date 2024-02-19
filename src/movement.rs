use crate::CELL_SIZE;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub dir: Direction,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_postion);
    }
}

#[derive(Component)]
pub struct Velocity {
    value: f32,
}

impl Velocity {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Direction {
    pub current: Directions,
    pub next: Directions,
}

impl Direction {
    pub fn new(current: Directions, next: Directions) -> Self {
        Self { current, next }
    }
}

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn get_transform(&self) -> Transform {
        Transform::from_xyz(
            self.x * CELL_SIZE + (CELL_SIZE / 2.),
            -(CELL_SIZE / 2.) - self.y * CELL_SIZE,
            0.,
        )
    }
}

impl From<&Position> for Transform {
    fn from(value: &Position) -> Self {
        value.get_transform()
    }
}

fn update_postion(
    mut query: Query<(&Velocity, &Direction, &mut Position, &mut Transform)>,
    time: Res<Time>,
) {
    for (velocity, direction, mut position, mut transform) in query.iter_mut() {
        let (delta_x, delta_y) = match direction.current {
            Directions::Up => (0., 0. - velocity.value),
            Directions::Down => (0., 0. + velocity.value),
            Directions::Left => (0. - velocity.value, 0.),
            Directions::Right => (0. + velocity.value, 0.),
        };
        position.x += delta_x * time.delta_seconds();
        position.y += delta_y * time.delta_seconds();
        *transform = position.get_transform();
    }
}

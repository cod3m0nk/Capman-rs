use crate::{
    board::{Board, CellType},
    player::Player,
    CELL_SIZE,
};
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
        app.add_systems(Update, update_player_position);
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

#[derive(PartialEq, Eq, Clone, Copy)]
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

    fn get_neighbour(&self, dir: Directions) -> Self {
        let (mut dest_x, mut dest_y) = (self.x, self.y);
        match dir {
            Directions::Up => dest_y = dest_y.ceil() - 1.,
            Directions::Down => dest_y = dest_y.floor() + 1.,
            Directions::Left => dest_x = dest_x.ceil() - 1.,
            Directions::Right => dest_x = dest_x.floor() + 1.,
        };
        Position {
            x: dest_x,
            y: dest_y,
        }
    }
}

impl From<&Position> for Transform {
    fn from(value: &Position) -> Self {
        value.get_transform()
    }
}

fn update_player_position(
    mut query: Query<(
        &Velocity,
        &mut Direction,
        &mut Position,
        &mut Transform,
        &Player,
    )>,
    time: Res<Time>,
    board: Res<Board>,
) {
    let (velocity, mut direction, mut position, mut transform, _) = query.single_mut();

    let distance = velocity.value * time.delta_seconds();
    move_player(&mut direction, &mut position, &board, distance);
    *transform = position.get_transform();
}

fn move_player(direction: &mut Direction, position: &mut Position, board: &Board, distance: f32) {
    let mut distance = distance;
    while distance > 0. {
        update_direction(position, direction, board);
        let dest = position.get_neighbour(direction.current);
        if matches!(board.get_cell(dest.x, dest.y), CellType::Wall(_)) {
            break;
        }
        distance = update_position(position, &dest, distance);
    }
}

fn update_direction(position: &Position, direction: &mut Direction, board: &Board) {
    if direction.current == direction.next
        || (position.x.floor() - position.x).abs() > f32::EPSILON
        || (position.y.floor() - position.y).abs() > f32::EPSILON
    {
        return;
    }

    let next = position.get_neighbour(direction.next);
    if !matches!(board.get_cell(next.x, next.y), CellType::Wall(_)) {
        direction.current = direction.next;
    }
}

fn update_position(position: &mut Position, dest: &Position, distance: f32) -> f32 {
    // Calculate the delta distance to the destination
    let (delta_x, delta_y) = (dest.x - position.x, dest.y - position.y);

    // The movement is enough to reach the destination
    let delta = (delta_x + delta_y).abs();
    if delta < distance {
        position.x = dest.x;
        position.y = dest.y;
        return distance - delta;
    }

    if delta_y.abs() > f32::EPSILON {
        position.y += delta_y.signum() * distance;
    } else {
        position.x += delta_x.signum() * distance;
    }
    0.
}

use crate::{
    board::{Board, CellType},
    enemies::Enemy,
    player::{Player, PlayerState},
    state::GameState,
    CELL_SIZE,
};
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_player_position.run_if(in_state(GameState::Running)),
                update_enemy_position.run_if(in_state(GameState::Running)),
            ),
        );
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub dir: Direction,
}

pub trait MovableObject {
    fn update_direction(&self, pos: &Position, dir: &mut Direction, board: &Board);
}

#[derive(Component)]
pub struct Velocity {
    value: f32,
}

impl Velocity {
    pub const fn new(value: f32) -> Self {
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

impl Directions {
    pub fn iterator() -> std::slice::Iter<'static, Self> {
        static DIRECTIONS: [Directions; 4] = [
            Directions::Up,
            Directions::Right,
            Directions::Down,
            Directions::Left,
        ];
        DIRECTIONS.iter()
    }
}

#[derive(Component, PartialEq, Eq)]
pub struct Direction {
    pub current: Directions,
    pub next: Directions,
}

impl Direction {
    pub const fn new(current: Directions, next: Directions) -> Self {
        Self { current, next }
    }

    pub const fn is_opposite(&self, other: Directions) -> bool {
        matches!(
            (self.current, other),
            (Directions::Up, Directions::Down)
                | (Directions::Down, Directions::Up)
                | (Directions::Left, Directions::Right)
                | (Directions::Right, Directions::Left)
        )
    }
}

#[derive(Component, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn get_transform(&self) -> Transform {
        Transform::from_xyz(
            self.x.mul_add(CELL_SIZE, CELL_SIZE / 2.),
            self.y.mul_add(-CELL_SIZE, -(CELL_SIZE / 2.)),
            0.,
        )
    }

    pub fn write_into(&self, transform: &mut Transform) {
        transform.translation.x = self.x.mul_add(CELL_SIZE, CELL_SIZE / 2.);
        transform.translation.y = self.y.mul_add(-CELL_SIZE, -(CELL_SIZE / 2.));
    }

    pub fn get_target_cell(&self, dir: Directions) -> Self {
        let (mut dest_x, mut dest_y) = (self.x, self.y);
        match dir {
            Directions::Up => dest_y = dest_y.ceil() - 1.,
            Directions::Down => dest_y = dest_y.floor() + 1.,
            Directions::Left => dest_x = dest_x.ceil() - 1.,
            Directions::Right => dest_x = dest_x.floor() + 1.,
        };
        Self {
            x: dest_x,
            y: dest_y,
        }
    }

    /// This function is used to check if the coordinates are aligned on a grid.
    ///  The function returns true if the coordinates are aligned precisely on
    ///  the grid, otherwise false.
    ///
    /// ## Arguments
    /// * `self` - A reference to the object containing x and y coordinates.
    ///
    /// ## Returns
    /// * `bool` - Indicates if the coordinates are aligned on the grid.
    ///
    pub fn is_grid_aligned(&self) -> bool {
        !((self.x.floor() - self.x).abs() > f32::EPSILON
            || (self.y.floor() - self.y).abs() > f32::EPSILON)
    }

    pub fn get_cell_coords(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn get_distance(&self, other: &Self) -> f32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    let (velocity, mut direction, mut position, mut transform, player) = query.single_mut();
    let start_pos = Position::new(position.x, position.y);

    let distance = velocity.value * time.delta_seconds();
    move_object(&mut direction, &mut position, &board, distance, player);
    position.write_into(&mut transform);

    if start_pos == *position {
        next_state.set(PlayerState::Idle);
    } else {
        next_state.set(PlayerState::Moving);
    }
}

fn update_enemy_position(
    mut query: Query<(
        &Velocity,
        &mut Direction,
        &mut Position,
        &mut Transform,
        &Enemy,
    )>,
    time: Res<Time>,
    board: Res<Board>,
) {
    for enemy in &mut query {
        let (velocity, mut direction, mut position, mut transform, enemy) = enemy;

        let distance = velocity.value * time.delta_seconds();
        move_object(&mut direction, &mut position, &board, distance, enemy);
        position.write_into(&mut transform);
    }
}

fn move_object(
    direction: &mut Direction,
    position: &mut Position,
    board: &Board,
    distance: f32,
    object: &dyn MovableObject,
) {
    let mut distance = distance;
    while distance > 0. {
        if position.is_grid_aligned() {
            object.update_direction(position, direction, board);
        }
        let dest = position.get_target_cell(direction.current);
        if matches!(board.get_cell(&dest), CellType::Wall(_)) {
            break;
        }
        distance = update_position(position, &dest, distance);
        wrap_around(position, board);
    }
}

fn wrap_around(position: &mut Position, board: &Board) {
    let (_rows, columns) = board.get_dimensions();
    if position.x as isize == columns as isize {
        position.x -= columns as f32 + 1.0;
        return;
    }
    if (position.x as isize) < 0 {
        position.x += columns as f32 + 1.0;
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

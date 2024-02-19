use crate::{
    movement::Position,
    pickup::{Dot, PowerPill},
    CELL_SIZE,
};
use bevy::prelude::*;

const DEFAULT_LAYOUT: &str = include_str!("default_layout.txt");

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let board: Board = DEFAULT_LAYOUT.into();
        app.add_systems(Startup, spawn_board_components)
            .insert_resource(board);
    }
}

#[derive(Resource)]
pub struct Board {
    columns: isize,
    rows: isize,
    cells: Vec<CellType>,
}

#[derive(Component)]
pub enum CellType {
    Wall(WallType),
    Dot,
    PowerPill,
    Empty,
    Outside,
}

pub enum WallType {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn spawn_board_components(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
) {
    for (index, cell_type) in board.cells.iter().enumerate() {
        let Some(texture) = cell_type.get_texture() else {
            continue;
        };
        let row = index as isize / board.columns;
        let column = index as isize % board.columns;

        let position = Position::new(column as f32, row as f32);
        let transform = position.get_transform();

        let sprite_bundle = SpriteBundle {
            texture: asset_server.load(texture),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::Center,
                rect: Some(Rect::new(0., 0., CELL_SIZE, CELL_SIZE)),
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                ..Default::default()
            },
            transform,
            ..Default::default()
        };

        match cell_type {
            CellType::Wall(_) => commands.spawn((position, sprite_bundle)),
            CellType::Dot => commands.spawn((sprite_bundle, Dot)),
            CellType::PowerPill => commands.spawn((sprite_bundle, PowerPill)),
            _ => continue,
        };
    }
}

impl Board {
    pub fn get_cell(&self, x: f32, y: f32) -> &CellType {
        if x < 0. || x >= self.columns as f32 {
            return &CellType::Outside;
        }
        if y < 0. || y >= self.rows as f32 {
            return &CellType::Outside;
        }
        &self.cells[y as usize * self.columns as usize + x as usize]
    }
    pub const fn get_dimensions(&self) -> (usize, usize) {
        (self.rows as usize, self.columns as usize)
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        fn get_wall_type(input: &str, row: isize, column: isize, columns: isize) -> WallType {
            let up = get_char(input, row - 1, column, columns);
            let down = get_char(input, row + 1, column, columns);
            let left = get_char(input, row, column - 1, columns);
            let right = get_char(input, row, column + 1, columns);
            match (up, down, left, right) {
                (_, '|', _, '-') => WallType::TopLeft,
                (_, '|', '-', _) => WallType::TopRight,
                ('|', _, '-', _) => WallType::BottomRight,
                ('|', _, _, '-') => WallType::BottomLeft,
                (_, '|', _, '+') | (_, '+', _, '-') => WallType::TopLeft,
                (_, '|', '+', _) | (_, '+', '-', _) => WallType::TopRight,
                ('|', _, '+', _) | ('+', _, '-', _) => WallType::BottomRight,
                ('|', _, _, '+') | ('+', _, _, '-') => WallType::BottomLeft,
                _ => {
                    unreachable!("Invalid wall type {column} {row} u{up} d{down} l{left} r{right}",)
                }
            }
        }

        fn get_char(input: &str, row: isize, column: isize, columns: isize) -> char {
            // At this point input still has breakline charactes at the end of
            // each row. We need to add 1 to each row to account for that while
            // calculating the index
            let index = (row * (columns + 1) + column) as usize;
            if column >= columns || column < 0 || row < 0 || index > input.len() {
                return ' ';
            }
            input.chars().nth(index).unwrap_or(' ')
        }

        let mut board = Self {
            columns: 0,
            rows: 0,
            cells: Vec::with_capacity(input.len()),
        };

        for line in input.lines() {
            if !line.contains(['+', '-', '|', 'O', '.']) {
                continue;
            }
            if board.rows == 0 {
                board.columns = line.len() as isize;
            }
            assert!(
                board.columns == line.len() as isize,
                "Invaid board line: {}",
                board.rows
            );
            for (column, char) in line.chars().enumerate() {
                let cell_type = match char {
                    '+' => {
                        let wall_type =
                            get_wall_type(input, board.rows, column as isize, board.columns);
                        CellType::Wall(wall_type)
                    }
                    '-' => CellType::Wall(WallType::Horizontal),
                    '|' => CellType::Wall(WallType::Vertical),
                    'O' => CellType::PowerPill,
                    '.' => CellType::Dot,
                    ' ' => CellType::Empty,
                    cell => {
                        unreachable!("invalid board cell: \"{cell}\"");
                    }
                };
                board.cells.push(cell_type);
            }
            board.rows += 1;
        }
        board
    }
}

impl CellType {
    const fn get_texture(&self) -> Option<&'static str> {
        match self {
            Self::Wall(WallType::Vertical) => Some("sprites/vertical.png"),
            Self::Wall(WallType::Horizontal) => Some("sprites/horizontal.png"),
            Self::Wall(WallType::TopLeft) => Some("sprites/top-left.png"),
            Self::Wall(WallType::TopRight) => Some("sprites/top-right.png"),
            Self::Wall(WallType::BottomLeft) => Some("sprites/bottom-left.png"),
            Self::Wall(WallType::BottomRight) => Some("sprites/bottom-right.png"),
            Self::Dot => Some("sprites/dot.png"),
            Self::PowerPill => Some("sprites/powerpill.png"),
            Self::Empty | Self::Outside => None,
        }
    }
}

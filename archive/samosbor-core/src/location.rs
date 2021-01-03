use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::marker::Copy;
use uuid::Uuid;

// ############ Components

/// Mark - this tile is occupied and nothing can step into
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Block ();

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PositionDelta {
    pub x: i32,
    pub y: i32,
}

/// WINDROSE
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Hash, Clone, Copy)]
pub struct Unit(pub Uuid);

// ############ Resources
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Tile {
    Wall, Floor
}

/// Location and location content
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

pub fn gen_map(width: usize, height: usize) -> Vec<Tile> {
    let mut tiles = vec![Tile::Floor; width * height];
    for i in 0..width {
        tiles[i] = Tile::Wall;
    }
    let left_bottom = (height - 1) * width;
    let right_bottom = height * width;
    for i in left_bottom..right_bottom {
        tiles[i] = Tile::Wall;
    }
    for i in 0..height {
        tiles[i*width] = Tile::Wall;
        tiles[i*width + width - 1] = Tile::Wall;
    }
    tiles
}

impl Location {
    pub fn new(width: usize, height: usize) -> Location {
        let tiles = gen_map (width, height);
        Location {
            width,
            height,
            tiles,
        }
    }
}

// ############ Logic

/// Get index of given x,y in single-dimension map array
pub fn xy2idx(width: usize, x: usize, y: usize) -> usize {
    (y * width) + x
}

/// Transform position to index in single-dimension map array
pub fn pos2idx(width: usize, pos: Position) -> usize {
    xy2idx(width, pos.x, pos.y)
}

/// Get x,y from index in single-dimension map array
pub fn idx2xy(width: usize, idx: usize) -> (usize, usize) {
    (idx % width, idx / width)
}

/// Get x,y from index in single-dimension map array
pub fn idx2pos(width: usize, idx: usize) -> Position {
    let (x,y) = idx2xy(width, idx);
    Position {x, y}
}

pub fn dir2delta(dir: Direction) -> PositionDelta {
    match dir {
        Direction::N  => PositionDelta {x:  0, y: -1},
        Direction::NE => PositionDelta {x:  1, y: -1},
        Direction::E  => PositionDelta {x:  1, y:  0},
        Direction::SE => PositionDelta {x:  1, y:  1},
        Direction::S  => PositionDelta {x:  0, y:  1},
        Direction::SW => PositionDelta {x: -1, y:  1},
        Direction::W  => PositionDelta {x: -1, y:  0},
        Direction::NW => PositionDelta {x: -1, y: -1},
    }
}

pub fn apply_delta(pos: Position, delta: PositionDelta) -> Position {
    let x = pos.x as i32 + delta.x;
    let y = pos.y as i32 + delta.y;
    if x < 0 || y < 0 { pos } else {
        Position {x: x as usize, y: y as usize}
    }
}

pub fn eval_direction(pos: Position, dir: Direction) -> Position {
    apply_delta(pos, dir2delta(dir))
}

pub fn rectangle_tiles(
    left_top: Position,
    right_bottom: Position,
) -> Vec<Position> {
    let mut result = Vec::new();
    for y in left_top.y..=right_bottom.y {
        for x in left_top.x..=right_bottom.x {
            result.push(Position {x:x, y:y})
        }
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos2idx_idx2pos() {
        assert_eq!(
            pos2idx(10, Position{x:0, y:0}),
            0
        );
        assert_eq!(
            pos2idx(10, Position{x:0, y:1}),
            10
        );
        assert_eq!(
            pos2idx(10, Position{x:1, y:1}),
            11
        );

        assert_eq!(
            pos2idx(64, idx2pos(64, 2048)),
            2048
        );
    }

    #[test]
    fn test_eval_direction() {
        // Try to go under zero on any of axes
        assert_eq!(
            eval_direction(Position{x:0, y:0}, Direction::N),
            Position{x:0, y:0},
        );
        assert_eq!(
            eval_direction(Position{x:0, y:0}, Direction::W),
            Position{x:0, y:0},
        );
        assert_eq!(
            eval_direction(Position{x:0, y:1}, Direction::N),
            Position{x:0, y:0},
        );
        assert_eq!(
            eval_direction(Position{x:1, y:0}, Direction::W),
            Position{x:0, y:0},
        );
        assert_eq!(
            eval_direction(Position{x:1, y:1}, Direction::NW),
            Position{x:0, y:0},
        );
        assert_eq!(
            eval_direction(Position{x:0, y:0}, Direction::NW),
            Position{x:0, y:0},
        );

        // Just moving
        assert_eq!(
            eval_direction(Position{x:0, y:0}, Direction::S),
            Position{x:0, y:1},
        );
    }
}

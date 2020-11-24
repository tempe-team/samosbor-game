use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::marker::Copy;
use uuid::Uuid;
use bracket_lib::prelude::*;

// Constants
const NORTH: PositionDelta
    = PositionDelta {x:  0, y: -1};
const NORTH_EAST: PositionDelta
    = PositionDelta {x:  1, y: -1};
const EAST: PositionDelta
    = PositionDelta {x:  1, y:  0};
const SOUTH_EAST: PositionDelta
    = PositionDelta {x:  1, y:  1};
const SOUTH: PositionDelta
    = PositionDelta {x:  0, y:  1};
const SOUTH_WEST: PositionDelta
    = PositionDelta {x: -1, y:  1};
const WEST: PositionDelta
    = PositionDelta {x: -1, y:  0};
const NORTH_WEST: PositionDelta
    = PositionDelta {x: -1, y: -1};

const STEP_COST_DIRECT: f32 = 1.0;
const STEP_COST_DIAGONAL: f32 = 1.5;

// Components

/// Mark - this tile is occupied and nothing can step into
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Block ();

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Copy)]
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
    pub passable: Vec<bool>,
}

impl Location {
    pub fn new(width: usize, height: usize) -> Location {
        let (tiles, passable) = gen_map (width, height);
        Location {
            width,
            height,
            tiles,
            passable,
        }
    }

    pub fn try_to_spawn_blocking (&mut self) -> Option<Position> {
        let width = self.width;
        let mbidx = self.passable.iter().enumerate()
            .filter(|(_, val)| **val)
            .next();
        match mbidx {
            Some ((free_idx, _)) => {
                self.passable[free_idx] = false;
                Some (idx2pos(width, free_idx))
            },
            _ => None,
        }
    }

    pub fn block_tile_by_position (&mut self, pos: Position) {
        let idx = pos2idx(self.width, pos);
        self.passable [idx] = false;
    }

    pub fn move_blocking_thing (
        &mut self,
        old_pos: Position,
        new_pos: Position,
    ) -> Result<(), ()> {
        let width = self.width;
        let old_idx = pos2idx(width, old_pos);
        let new_idx = pos2idx(width, new_pos);
        if self.passable[new_idx] {
            self.passable[new_idx] = false;
            self.passable[old_idx] = true;
            Ok (())
        } else {
            Err(())
        }
    }

    pub fn unblock_tile (
        &mut self,
        pos: Position,
    ) {
        let width = self.width;
        self.passable[pos2idx(width, pos)] = true;
    }
}

impl Algorithm2D for Location {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Location {
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx as usize] == Tile::Wall
    }
    fn get_available_exits(
        &self,
        idx: usize,
    ) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let pos = idx2pos(self.width, idx);

        for delta in [
            EAST,
            WEST,
            NORTH,
            SOUTH,
        ].iter() {
            match apply_delta(pos, *delta) {
                Some(new_pos) => {
                    let idx = pos2idx(self.width, new_pos);
                    if self.passable[idx] {
                        exits.push((
                            idx,
                            STEP_COST_DIRECT
                        ))
                    }
                },
                None => ()
            }
        }

        for delta in [
            NORTH_EAST,
            SOUTH_EAST,
            SOUTH_WEST,
            NORTH_WEST,
        ].iter() {
            match apply_delta(pos, *delta) {
                Some(new_pos) => {
                    let idx = pos2idx(self.width, new_pos);
                    if self.passable[idx] {
                        exits.push((
                            idx,
                            STEP_COST_DIAGONAL,
                        ))
                    }
                },
                None => ()
            }
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1),
            self.index_to_point2d(idx2)
        )
    }
}

pub fn gen_map(width: usize, height: usize) -> (Vec<Tile>, Vec<bool>) {
    let mut tiles    = vec![Tile::Floor; width * height];
    let mut passable = vec![true; width * height];
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
    for (i, t) in tiles.iter().enumerate() {
        match t {
            Tile::Floor => passable[i] = true,
            Tile::Wall => passable[i] = false,
        }
    }
    (tiles, passable)
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
        Direction::N  => NORTH,
        Direction::NE => NORTH_EAST,
        Direction::E  => EAST,
        Direction::SE => SOUTH_EAST,
        Direction::S  => SOUTH,
        Direction::SW => SOUTH_WEST,
        Direction::W  => WEST,
        Direction::NW => NORTH_WEST,
    }
}

pub fn delta2dir(delta: PositionDelta) -> Option<Direction> {
    match delta {
        NORTH      => Some (Direction::N),
        NORTH_EAST => Some (Direction::NE),
        EAST       => Some (Direction::E),
        SOUTH_EAST => Some (Direction::SE),
        SOUTH      => Some (Direction::S),
        SOUTH_WEST => Some (Direction::SW),
        WEST       => Some (Direction::W),
        NORTH_WEST => Some (Direction::NW),
        _ => None,
    }
}

pub fn apply_delta(
    pos: Position,
    delta: PositionDelta,
) -> Option<Position> {
    let x = pos.x as i32 + delta.x;
    let y = pos.y as i32 + delta.y;
    if x < 0 || y < 0 {
        None
    } else {
        Some (
            Position {
                x: x as usize,
                y: y as usize,
            }
        )
    }
}

/// Get spatial direction from two points if they are neighbors
pub fn idxs2dir(
    width: usize,
    from: usize,
    to: usize,
) -> Option<Direction> {
    let from_pos = idx2pos(width, from);
    let to_pos = idx2pos(width, to);
    coords2dir(from_pos, to_pos)
}

/// Get spatial direction from two points if they are neighbors
pub fn coords2dir(
    from: Position,
    to: Position,
) -> Option<Direction> {
    delta2dir(
        PositionDelta {
            x: to.x as i32 - from.x as i32,
            y: to.y as i32 - from.y as i32,
        }
    )
}

pub fn eval_direction(
    pos: Position,
    dir: Direction,
) -> Option<Position> {
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
    fn test_coords2dir() {
        assert_eq!(
            coords2dir(
                width: 3,
                from: Position {x:2, y:2},
                to: Position{x:3, y:3},
            ),
            Direction::SE,
        )
    }

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

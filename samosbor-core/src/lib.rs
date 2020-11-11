use legion::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::string::{String};
use std::str::FromStr;
use std::fmt::{Display};

pub mod location;
pub mod protocol;
pub mod render;
pub mod serialization;

use crate::location::{
    Block,
    Location,
    Position,
    Unit,
    Tile,
    Direction,
    eval_direction,
    pos2idx,
    rectangle_tiles,
    idx2pos,
};
use crate::protocol::{
    Event,
    Event::{
        Step,
        AddUnit,
        RemoveUnit,
        ClientConnect,
        ClientDisconnect,
    },
    SamosborError,
    SamosborError::{
        Collision,
        NoSuchUnit,
    },
    SamosborMessage,
    SamosborMessage::{SmsbrEvent, SmsbrError, SmsbrState},
};
use crate::render::{Renderable, get_segment};
use crate::serialization::{SerializeMe, serialize_state};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct ClientId(Uuid); // Id which know only client and server

impl ClientId {
    pub fn new(client_id: uuid::Uuid) -> ClientId {
        ClientId(client_id)
    }
}
impl FromStr for ClientId {
    type Err = uuid::Error;

    fn from_str(uuid_str: &str) -> std::result::Result<Self, uuid::Error> {
        Ok(ClientId(Uuid::parse_str(uuid_str)?))
    }
}

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        match self {
            ClientId (uuid) => uuid.fmt(f),
        }
    }
}

/// Prototype render - should draw world part of given
/// window size depending from point of view
pub fn display_world_segment(
    world: &World,
    resources: &Resources,
    view_point: Position,
    window_radius: usize,
) -> String {
    let mb_location = resources.get::<Location>();
    match mb_location {
        Some(location) => {
            let mut rendered_world = Vec::new();
            location
                .tiles.iter()
                .for_each(|&val|{
                    match val {
                        Tile::Wall => rendered_world.push('#'),
                        Tile::Floor => rendered_world.push('.'),
                    }
                }); // TODO: render whole world for segment display?
            let mut query = <(&Position, &Renderable)>::query();
            query
                .iter(world)
                .for_each(|(pos, ren)|{
                    rendered_world[
                        pos2idx(location.width, *pos)
                    ] = ren.glyph;
                });
            get_segment(
                location.width,
                location.height,
                window_radius,
                view_point,
                rendered_world,
            )
        },
        None => unreachable!(),
    }
}

pub fn spawn_unit(
    unit: Unit,
    world: &mut World,
    resources: &mut Resources,
) -> Result<Position, SamosborError> {
    let mut location = resources
        .get_mut::<Location>()
        .ok_or(SamosborError::InternalLogicError)?;
    let pos = location
        .try_to_spawn_blocking()
        .ok_or(SamosborError::NoEmptyTiles)?;
    world.push((
                unit,
                pos.clone(),
                Renderable { glyph: '@'},
                Block (),
                SerializeMe ()));
    Ok(pos)
}

pub fn add_unit(
    world: &mut World,
    resources: &mut Resources,
    unit: Unit,
    pos: Position
) -> Result<(), SamosborError>  {
    let mut location = resources
        .get_mut::<Location>()
        .ok_or(SamosborError::InternalLogicError)?;
    location.block_tile_by_position(pos);
    world.push((
        unit,
        pos,
        Renderable {glyph: '@'},
        Block (),
        SerializeMe ()
    ));
    Ok(())
}

pub fn remove_unit(
    world: &mut World,
    unit: Unit,
) -> Result<(), SamosborError> {
    let mut query = <(Entity, &Unit)>::query();
    let to_remove = query
        .iter(world)
        .filter(|(_, unit_)|{ *unit_.clone() == unit})
        .map(|(entity, _)| *entity).collect::<Vec<Entity>>();
    for entity in to_remove {
            world.remove(entity);
    };
    Ok (())
}

pub fn get_first_free_tile(
    world: &mut World,
    resources: &Resources,
) -> Option<Position> {
    let location = resources.get::<Location>().unwrap();
    let mut query = <(&Block, &Position)>::query();
    let positions = rectangle_tiles(
        Position {x: 0, y: 0},
        Position {
            x: location.width - 1,
            y: location.height - 1,
        },
    );
    let mut result = None;
    for position in positions.iter() {
        let mb_free_tile = query
            .iter(world)
            .filter(|(_, position_)| *position_ == position
            ).next();
        match mb_free_tile {
            Some(_) => (),
            None => {
                result = Some(*position);
                break
            },
        }
    };
    result
}

/// Look which tiles on location is walls
pub fn block_tiles_from_location(
    world: &mut World,
    resources: &Resources,
) {
    let mb_location = resources.get::<Location>();
    match mb_location {
        Some(loc) => {
            loc
                .tiles
                .iter()
                .enumerate()
                .for_each(|(idx, mb_wall)| match mb_wall{
                    Tile::Wall => {
                        let _ = world.push((
                            Block(),
                            idx2pos(loc.width, idx),
                        ));
                    },
                    _ => ()
                })
        },
        None => (),
    }
}

pub fn unit_step(
    world: &mut World,
    resources: &mut Resources,
    unit: Unit,
    dir: Direction
) -> Result<(),SamosborError> {
    let mut get_position_query =
        <(&Unit, &Position)>::query();
    let mut update_query =
        <(&Unit, &mut Position)>::query();
    let mut location = resources
        .get_mut::<Location>()
        .ok_or(SamosborError::InternalLogicError)?;
    let (_, old_pos) = get_position_query
        .iter(world)
        .filter(|(unit_, _)| **unit_ == unit)
        .next().ok_or(NoSuchUnit)?;
    let dest_pos = eval_direction(*old_pos, dir);
    if let Err (_) = location.move_blocking_thing (
        *old_pos,
        dest_pos,
    ) {
        Err (Collision)
    } else {
        update_query
            .iter_mut(world)
            .filter(|(unit_, _)|{**unit_ == unit})
            .for_each (|(_, pos)|{
                {
                    pos.x = dest_pos.x;
                    pos.y = dest_pos.y;
                }
            });
        Ok(())
    }
}

                                           /// Apply event to state, mutate it, and get response
pub fn eval_event(
    world: &mut World,
    resources: &mut Resources,
    e:Event
) -> ( Option <SamosborMessage>, // message to client who is reason of event
       Option <SamosborMessage>, // message to rest clients
) {
    match e {
        AddUnit {unit, position} => match add_unit(world, resources, unit, position) {
            Ok(()) => (
                Some(SmsbrEvent(e)),
                Some(SmsbrEvent(e))
            ),
            Err(err) => (
                Some(SmsbrError(err)),
                None
            ),
        },
        RemoveUnit (unit) => match remove_unit(world, unit) {
            Ok(()) => (
                Some(SmsbrEvent(e)),
                Some(SmsbrEvent(e))
            ),
            Err(err) => (
                Some(SmsbrError(err)),
                None
            ),
        },
        Step {unit, direction} => match unit_step(world, resources, unit, direction) {
            Ok(()) => (
                Some(SmsbrEvent (e)),
                Some(SmsbrEvent (e)),
            ),
            Err(err) => (
                Some(SmsbrError (err)),
                None
            ),
        },
        ClientConnect(unit) => {
            let mbpos = spawn_unit(unit, world, resources);
            match mbpos {
                Ok (position) => {
                    let to_target = SmsbrState(
                        serde_json::to_value((
                            unit,
                            serialize_state(world, resources)
                        )).unwrap()
                    );
                    let to_others = SmsbrEvent(
                        AddUnit {
                            unit: unit,
                            position: position,
                        }
                    );
                    (Some(to_target), Some(to_others))
                }
                Err (err) => (Some(SmsbrError(err)), None),
            }
        }
        ClientDisconnect(unit) => match remove_unit(world, unit) {
            Ok(()) => (
                None,
                Some(SmsbrEvent(RemoveUnit(unit))),
            ),
            Err(err) => (
                Some (SmsbrError (err)),
                None,
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use location::Direction;

    #[test]
    fn test_unit_step() {
        let mut world = World::default();
        let unit = Unit (Uuid::new_v4());
        let _ = add_unit (
            &mut world,
            unit,
            Position {x:1, y:1}
        );
        let _ = unit_step(&mut world, unit, Direction::N);
        let _ = unit_step(&mut world, unit, Direction::W);
        let mut query = <(&Unit, &Position)>::query();
        let (runit, rposition) = query.iter(&world).nth(0).unwrap();
        assert_eq!(
            (*runit, *rposition),
            (unit, Position {x:0, y:0}),
        );
    }

    #[test]
    fn test_add_remove_unit() {
        let mut world = World::default();
        let unit = Unit (Uuid::new_v4());
        let _ = add_unit (&mut world, unit, Position {x:1, y:1});
        let mut query = <&Unit>::query();
        let result = query.iter(&world).nth(0).unwrap();
        assert_eq!(
            result,
            &unit,
        );
        let _ = remove_unit(&mut world, unit.clone());
        let mut query = <&Unit>::query();
        let result = query.iter(&world).nth(0);
        assert_eq!(
            result,
            None,
        );
    }

    #[test]
    fn test_display_world_segment() {
        let mut world = World::default();
        let mut resources = Resources::default();
        resources.insert(Location::new(3, 3));
        let unit = Unit (Uuid::new_v4());
        let _ = add_unit (&mut world, unit, Position {x:1, y:1});
        block_tiles_from_location(&mut world, &resources);
        let result = display_world_segment(
            &world,
            &resources,
            Position {x:1, y:1},
            1,
        );
        assert_eq!(
            result,
            "###\n#@#\n###\n",
        );
    }
}

use legion::*;
use serde::de::DeserializeSeed;
use serde_json;
use serde_json::value::Value;
use serde_json::map::Map;
use serde_json::error::Error;
use serde::{Deserialize, Serialize};

use crate::location::{
    Unit,
    Position,
    Location,
    Block,
};
use crate::render::{
    Renderable,
};

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SerializeMe();

pub fn serialize_state(world: &World, resources: &Resources) -> Value {
    let mut registry = Registry::<String>::default();
    registry.register::<Position>("position".to_string());
    registry.register::<Unit>("unit".to_string());
    registry.register::<Renderable>("renderable".to_string());
    registry.register::<Block>("block".to_string());
    registry.register::<SerializeMe>("serializable".to_string());

    let world_serialized = serde_json::to_value(world.as_serializable(component::<SerializeMe>(), &registry)).unwrap();
    let map_serialized = match resources.get::<Location>() {
        Some (location) => serde_json::to_value(location.clone()).unwrap(),
        None => unreachable!(),
    };
    let mut result = Map::new();
    result.insert("world".to_string(), world_serialized);
    result.insert("map".to_string(), map_serialized);
    Value::Object(result)
}

fn stringify(x: Error) -> String { format!("error: {}", x) }

pub fn deserialize_state(
    json: Value
) -> Result<(World, Resources), String> {
    match json {
        Value::Object (state_json) => {
            let world_value = state_json.get(&"world".to_string()).ok_or("No world here".to_string())?;
            let map_value = state_json.get(&"map".to_string()).ok_or("No map here".to_string())?;
            let mut registry = Registry::<String>::default();
            registry.register::<Position>("position".to_string());
            registry.register::<Unit>("unit".to_string());
            registry.register::<Renderable>("renderable".to_string());
            registry.register::<Block>("block".to_string());
            registry.register::<SerializeMe>("serializable".to_string());
            let world = registry.as_deserialize().deserialize(world_value).map_err(stringify)?;
            let location:Location = serde_json::from_value(
                map_value.clone()
            ).map_err(stringify)?;
            let mut resources = Resources::default();
            resources.insert(location);
            Ok((world, resources))
        },
        _ => Err("Invalid object".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_serialize_deserialize() {
        let mut world = World::default();
        let mut resources = Resources::default();
        resources.insert(Location::new(1, 1));

        let unit = Unit (Uuid::new_v4());
        let pos = Position {x:1, y:1};
        let symbol = Renderable { glyph: '@'};
        let ser_mark = SerializeMe();
        world.push((
            unit,
            pos,
            symbol,
            ser_mark,
        ));

        let serialized = serialize_state (&world, &resources);
        let _ = deserialize_state(serialized.clone()).unwrap(); // panics if deserialization have no success
    }
}

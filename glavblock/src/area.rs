use std::collections::{
    HashMap,
};
use std::ops::*;
use std::iter::FromIterator;

use legion::*;

/// Виды помещений
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum AreaType {
    Living, // жилячейки
    Science, // лаборатории
    Military, // казармы
    Industrial, // технические и производственные помещения. терминалы, распределительные узлы, насосы, чаны, станки.
    Party, // склады, образовательные помещения, детские сады, школы, залы партсобраний
}


/// Вместимость помещения(единицы площади)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AreaCapacity(pub usize);

impl From<AreaCapacity> for usize {
    fn from(val: AreaCapacity) -> usize {
        val.0
    }
}

/// Занятая площадь(единицы площади)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, )]
pub struct AreaOccupied(pub usize);

impl From<AreaOccupied> for usize {
    fn from(val: AreaOccupied) -> usize {
        val.0
    }
}

impl AddAssign for AreaOccupied {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for AreaOccupied {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl From<AreaCapacity> for AreaOccupied {
    fn from(val: AreaCapacity) -> AreaOccupied {
        AreaOccupied (val.0)
    }
}

/// Метка того, к какой комнате принадлежит эта штука
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BelongsToRoom (pub Entity);

/// Узнать сколько в комнате осталось места
pub fn get_room_free_space(
    world: &mut World,
    room: Entity,
) -> i32 { // может быть негативным
    let AreaCapacity (capacity) = world
        .entry(room)
        .unwrap()
        .into_component::<AreaCapacity>()
        .unwrap()
        .clone();
    let mut query = <(
        &BelongsToRoom,
        &AreaOccupied
    )>::query();
    let mut sum:usize = 0;
    for occupied in query.iter(world)
        .filter(
            |(&BelongsToRoom (entity), _)|
            entity == room
        ).map(|tup|tup.1) {
            let occupied_:usize = occupied.clone().into();
            sum += occupied_;
        };
    capacity as i32 - sum as i32
}

/// Есть ли у нас комната этого назначения
/// в которую вместится нечто указанного размера
pub fn get_sufficent_room(
    world: &mut World,
    for_: AreaOccupied,
    type_: AreaType,
) -> Option<Entity> {
    let mut areas: HashMap<Entity, (AreaCapacity, AreaOccupied)> = HashMap::new();
    let mut areasq = <(
        &AreaType,
        &AreaCapacity,
        &Entity,
    )>::query();
    for (_, capacity, entity) in areasq
        .iter(world)
        .filter(|(artype,_, _)| **artype == type_)
    {
        areas.insert(*entity, (*capacity, AreaOccupied(0)));
    }

    let mut buildingsq = <(
        &BelongsToRoom,
        &AreaOccupied,
    )>::query();

    // Собираем заполненность помещений
    for (room, building_size) in buildingsq.iter(world) {
        match areas.get_mut(&room.0) {
            Some((_, occupied)) => *occupied += *building_size,
            None => (),
        }
    }

    // FIXME: сравнение capacity >= occupied это обход переполнения
    let mut areas_free_space = Vec::from_iter(
        areas
            .iter()
            .filter(|(_, (c, o))| AreaOccupied::from(*c) >= *o)
            .map(|(k, (c, o))| (k, AreaOccupied::from(*c) - *o))
            .filter (|(_, o)| *o >= for_)
    );

    // берем наиболее забитые помещения
    // но в которые тем не менее вместится то что нам надо
    areas_free_space.sort_by (|(_, o1), (_, o2)| o1.cmp(o2));
    match areas_free_space.pop () {
        Some((e, _)) => Some (*e),
        None => None,
    }
}

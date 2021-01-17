use legion::*;
use crate::core::*;

/// Виды помещений
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum AreaType {
    Living, // жилячейки
    Science, // лаборатории
    Military, // казармы
    Industrial, // технические и производственные помещения. терминалы, распределительные узлы, насосы, чаны, станки.
    Party, // склады, образовательные помещения, детские сады, школы, залы партсобраний
    Medical, // медпункты, операционные
}


/// Вместимость помещения
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AreaCapacity(pub usize);

impl From<AreaCapacity> for usize {
    fn from(val: AreaCapacity) -> usize {
        val.0
    }
}

/// Сколько места занимает объект
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AreaOccupied(pub usize);

impl From<AreaOccupied> for usize {
    fn from(val: AreaOccupied) -> usize {
        val.0
    }
}

/// Метка того, к какому стационарному объекту принадлежит эта штука
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BelongsToStationary (pub Entity);

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
        .unwrap().clone();
    let mut query = <(&BelongsToRoom, &AreaOccupied)>::query();
    let mut sum:usize = 0;
    for mut occupied in query.iter(world)
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
/// в которой есть столько места
pub fn is_there_sufficent_room(
    size: usize,
    type_: AreaType,
) -> Result<Entity, SamosborError> {
    unimplemented!();
}

use std::cmp::min;
use std::ops::*;
use std::collections::HashMap;

use legion::*;

use crate::area::*;
use crate::core::*;
use crate::resources::*;

/// Вместимость контейнера(единицы объема)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VolumeCapacity (pub usize);

/// Занятое место (единицы объема)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VolumeOccupied (pub usize);

/// Для случаев когда надо заполнить или опустошить контейнер
/// на весь доступный объем
impl From<VolumeCapacity> for VolumeOccupied {
    fn from(val: VolumeCapacity) -> VolumeOccupied {
        VolumeOccupied (val.0)
    }
}

impl Add for VolumeOccupied {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl SubAssign for VolumeOccupied {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

/// Вещественные единицы (количество ресурса)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RealUnits (pub usize);

impl SubAssign for RealUnits {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl AddAssign for RealUnits {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for RealUnits {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

/// Стандартная вместимость контейнера(полки)
/// в единицах объема
const CONTAINER_VOLUME:VolumeCapacity = VolumeCapacity (1000);

/// Сколько места занимает напольный контейнер
const CONTAINER_SIZE:AreaOccupied = AreaOccupied (5);

/// Тип ресурса. Текучий или твердый.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StorageType {
    Solid,
    Fluid,
}

/// Полка на стеллаже
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shelf ();

/// Чан (как контейнер для хранения, не как постройка)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Barrel ();

/// Контейнер на полу
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Container ();

/// Тип хранения ресурса
fn container_type (
    resource: Resource
) -> StorageType {
    match resource {
        Resource::BioRawT1           => StorageType::Solid,
        Resource::BioRawT2           => StorageType::Solid,
        Resource::BioRawT3           => StorageType::Solid,
        Resource::ScrapT1            => StorageType::Solid,
        Resource::ScrapT2            => StorageType::Solid,
        Resource::ScrapT3            => StorageType::Solid,
        Resource::Concrete         => StorageType::Solid,
        Resource::IsoConcrente     => StorageType::Solid,
        Resource::TransparentSlime => StorageType::Fluid,
        Resource::BlackSlime       => StorageType::Fluid,
        Resource::BrownSlime       => StorageType::Fluid,
        Resource::RedSlime         => StorageType::Fluid,
        Resource::PinkSlime        => StorageType::Fluid,
        Resource::WhiteSlime       => StorageType::Fluid,
        Resource::ComponentT1      => StorageType::Solid,
        Resource::ComponentT2      => StorageType::Solid,
        Resource::ComponentT3      => StorageType::Solid,
        Resource::ReagentT1        => StorageType::Solid,
        Resource::ReagentT2        => StorageType::Solid,
        Resource::ReagentT3        => StorageType::Solid,
        Resource::PolymerT1        => StorageType::Solid,
        Resource::PolymerT2        => StorageType::Solid,
        Resource::PolymerT3        => StorageType::Solid,
        Resource::ConcentratT1     => StorageType::Solid,
        Resource::ConcentratT2     => StorageType::Solid,
        Resource::ConcentratT3     => StorageType::Solid,
    }
}

/// Каждый контейнер имеет вместимость 1000 объемных единиц.
/// контейнер занимает 5 единиц площади
/// Функция говорит сколько единиц объема занимает одна
/// вещественная единица конкретного ресурса.
pub fn piece_size (
    resource: Resource,
) -> VolumeOccupied {
    match resource {
        Resource::BioRawT1         => VolumeOccupied(100),
        Resource::BioRawT2         => VolumeOccupied(100),
        Resource::BioRawT3         => VolumeOccupied(100),
        Resource::ScrapT1          => VolumeOccupied(100),
        Resource::ScrapT2          => VolumeOccupied(100),
        Resource::ScrapT3          => VolumeOccupied(100),
        Resource::Concrete         => VolumeOccupied(1000),
        Resource::IsoConcrente     => VolumeOccupied(100),
        Resource::TransparentSlime => VolumeOccupied(50),
        Resource::BlackSlime       => VolumeOccupied(50),
        Resource::BrownSlime       => VolumeOccupied(50),
        Resource::RedSlime         => VolumeOccupied(50),
        Resource::PinkSlime        => VolumeOccupied(50),
        Resource::WhiteSlime       => VolumeOccupied(50),
        Resource::ComponentT1      => VolumeOccupied(100),
        Resource::ComponentT2      => VolumeOccupied(100),
        Resource::ComponentT3      => VolumeOccupied(100),
        Resource::ReagentT1        => VolumeOccupied(100),
        Resource::ReagentT2        => VolumeOccupied(100),
        Resource::ReagentT3        => VolumeOccupied(100),
        Resource::PolymerT1        => VolumeOccupied(10),
        Resource::PolymerT2        => VolumeOccupied(10),
        Resource::PolymerT3        => VolumeOccupied(10),
        Resource::ConcentratT1     => VolumeOccupied(1),
        Resource::ConcentratT2     => VolumeOccupied(1),
        Resource::ConcentratT3     => VolumeOccupied(1),
    }
}

/// Какой объем нужен для хранения этого количества ресурса
pub fn real2volume(
    resource: Resource,
    amount: RealUnits,
) -> VolumeOccupied {
    let size = piece_size(resource);
    match (size, amount) {
        (VolumeOccupied (volume), RealUnits (units)) => VolumeOccupied(volume*units),
    }
}

/// Сколько вещественных единиц ресурса в этом объеме
pub fn volume2real(
    resource: Resource,
    volume: VolumeOccupied,
) -> RealUnits {
    let size = piece_size(resource);
    match (size, volume) {
        (VolumeOccupied (size_), VolumeOccupied (volume_)) => RealUnits(volume_ / size_),
    }
}

/// Хранить ресурс в чане.
/// Возвращает количество ресурса которое не удалось залить
/// на хранение. Если 0 - значит все залили.
fn store_in_barrels (
    world: &mut World,
    amount: RealUnits,
    resource: Resource,
) -> RealUnits {
    if amount == RealUnits (0) {
        return RealUnits (0)
    } else {
        let mut query = <(
            &Barrel,
            &mut Option<Resource>,
            &mut VolumeOccupied
        )>::query();
        let free_barrels = query
            .iter_mut(world)
            .filter(|(_, res, _)| **res == None);
        let mut not_deposited = amount;
        for (_, res, occupied) in free_barrels {
            *res = Some(resource);

            *occupied = min(
                VolumeOccupied::from(CONTAINER_VOLUME),
                real2volume(
                    resource,
                    not_deposited.clone(),
                ),
            );
            if occupied.clone() == VolumeOccupied (0) {
                // Все распределено
                break
            } else {
                // Что-то осталось.
                // Заходим на следующий виток.
                not_deposited -= volume2real(
                    resource,
                    occupied.clone(),
                );
            }
        }
        // Либо все распределили,
        // либо бочки закончились
        // и возвращаем сколько не разместили
        not_deposited
    }}

/// Хранить ресурс на полке.
/// Возвращает количество ресурса которое не удалось положить
/// на хранение. Если 0 - значит все разложили.
fn store_on_shelves (
    world: &mut World,
    amount: RealUnits,
    resource: Resource
) -> RealUnits {
    if amount == RealUnits (0) {
        return RealUnits (0)
    } else {
        let mut query = <(
            &Shelf,
            &mut Option<Resource>,
            &mut VolumeOccupied,
        )>::query();
        let free_shelves = query
            .iter_mut(world)
            .filter(|(_, res, _)| **res == None);
        let mut not_deposited = amount;
        for (_, res, occupied) in free_shelves {
            *res = Some(resource);
            *occupied = min(
                VolumeOccupied::from(CONTAINER_VOLUME),
                real2volume(
                    resource,
                    not_deposited.clone(),
                ),
            );
            if occupied.clone() == VolumeOccupied (0) {
                // Все распределено
                break
            } else {
                // Что-то осталось.
                // Заходим на следующий виток.
                not_deposited -= volume2real(
                    resource,
                    occupied.clone(),
                );
            }
        }
        // Либо все распределили
        // либо полки закончились
        //и возвращаем сколько не разместили
        not_deposited
}}

/// Хранить ресурсы в ящиках на полу
/// Возвращает количество ресурса которое не вместилось
fn store_on_floor (
    world: &mut World,
    amount: RealUnits,
    resource: Resource,
) -> RealUnits {
    if amount == RealUnits(0) {
        return RealUnits(0)
    } else {
        let mut not_deposited = amount;
        while not_deposited > RealUnits (0) {
            let mbroom = get_sufficent_room(
                world,
                CONTAINER_SIZE,
                AreaType::Party, // партийный склад!
            );
            match mbroom {
                None => break, // нет на складах места.
                Some (room) => {
                    let occupied = min(
                        VolumeOccupied::from(
                            CONTAINER_VOLUME
                        ),
                        real2volume(
                            resource,
                            not_deposited.clone(),
                        ),
                    );
                    not_deposited -= volume2real(
                        resource,
                        occupied.clone(),
                    );
                    world.push((
                        Container,
                        Some(resource),
                        BelongsToRoom(room),
                        occupied,
                    ));
                },
            }
        };
        not_deposited
    }
}

/// Положить ресурс на хранение.
/// Возвращает количество невместившегося ресурса.
pub fn put_resource(
    world: &mut World,
    resource: Resource,
    amount: RealUnits,
) -> RealUnits {
    // Тип хранения - твердый или текучий
    let container_type = container_type(resource);
    match container_type {
        // Твердые материалы можно хранить на полках и в кучах на полу
        StorageType::Solid => {
            let rest = store_on_shelves(
                world,
                amount,
                resource,
            );
            store_on_floor(world, rest, resource)
        },
        // Текучие материалы можно хранить только в чанах
        StorageType::Fluid => store_in_barrels(
            world,
            amount,
            resource,
        ),
    }
}

/// Забрать ресурс из контейнеров на полу.
/// Удалить контейнеры и освободить место.
/// Списание "с пола" всегда должно выполняться перед
/// списанием из мест хранения.
/// проверка наличия не проводится
/// Возвращает количество, которое не удалось забрать.
fn writeoff_from_floor (
    world: &mut World,
    resource: Resource,
    amount: RealUnits,
) -> RealUnits {
    let mut writeoff_query = <(
        &Option<Resource>,
        &Container,
        &Entity,
        &mut VolumeOccupied,
    )>::query();

    let mut empty_containers = Vec::new();
    let mut writed_off = RealUnits(0);
    let mut containers = writeoff_query
        .iter_mut(world)
        .filter(|(res, _, _, _)| **res == Some(resource))
        .map(|(_, _, e, v)| (e,v))
        .collect::<Vec<(
            &Entity,
            &mut VolumeOccupied,
        )>>();
    // здесь задумана сортировка
    // от более заполненных к менее
    // поменять местами выражение если не сработает
    containers.sort_by(
        |(_,occ1), (_, occ2)| occ2.cmp(occ1)
    );

    for (entity, occ) in containers.iter_mut() {
        let required_pieces = amount - writed_off;
        let required_volume = real2volume(
            resource,
            required_pieces,
        );
        // если в контейнере меньше чем надо
        // или ровно сколько надо
        if occ.clone () <= required_volume {
            // забираем из него все
            occ.0 = 0;
            writed_off += volume2real(resource, occ.clone());
            // планируем удаление контейнера
            empty_containers.push(entity.clone());
            // идем в следующий контейнер
        } else {
            // в контейнере больше чем надо
            // забираем оттуда требуемое количество
            **occ -= required_volume;
            writed_off += volume2real(
                resource,
                required_volume,
            );
            break
        }
    }
    for entity in empty_containers.iter() {
        world.remove (*entity);
    }
    amount - writed_off
}

/// Забрать ресурс из чанов или полок.
/// проверка наличия не проводится
/// Возвращает количество, которое не удалось забрать.
/// TODO: Количество которое не удалось забрать
/// должно быть гарантированно равно 0
fn writeoff_from_storage (
    world: &mut World,
    resource: Resource,
    amount: RealUnits,
) -> RealUnits {
    let mut writeoff_query = <(
        &mut Option<Resource>,
        &mut VolumeOccupied,
    )>::query();

    let mut writed_off = RealUnits(0);
    let mut containers = writeoff_query
        .iter_mut(world)
        .collect::<Vec<(
            &mut Option<Resource>,
            &mut VolumeOccupied,
        )>>();
    // здесь задумана сортировка
    // от более заполненных к менее
    // поменять местами выражение если не сработает
    containers.sort_by(
        |(_,occ1), (_, occ2)| occ2.cmp(occ1)
    );

    for (res, occ) in containers.iter_mut() {
        let required_pieces = amount - writed_off;
        let required_volume = real2volume(
            resource,
            required_pieces,
        );
        // если в контейнере меньше чем надо
        // или ровно сколько надо
        if **occ <= required_volume {
            // забираем из него все
            occ.0 = 0;
            writed_off += volume2real(resource, **occ);
            // чистим контейнер
            **res = None;
            // идем в следующий контейнер
        } else {
            // в контейнере больше чем надо
            // забираем оттуда требуемое количество
            **occ -= required_volume;
            writed_off += volume2real(
                resource,
                required_volume,
            );
            break
        }
    }
    amount - writed_off
}


/// Изъять ресурс, освободить пространство.
pub fn writeoff (
    world: &mut World,
    resource: Resource,
    amount: RealUnits,
) {
    let rest = writeoff_from_floor(
        world,
        resource,
        amount,
    );
    let rest_ = writeoff_from_storage(
        world,
        resource,
        rest,
    );
    assert_eq!(rest_, RealUnits(0));
}


/// сколько у нас на складах этого ресурса?
pub fn how_much_we_have (
    world: &mut World,
    resource: Resource,
) -> RealUnits {
    let mut deposit_query = <(
        &Option<Resource>,
        &VolumeOccupied
    )>::query();
    let deposit_volume = deposit_query
        .iter(world)
        .filter(|(res, _)| **res == Some(resource))
        .map(|(_, occ)| occ)
        .fold(VolumeOccupied(0), |a, b| a + *b);
    volume2real(resource, deposit_volume)
}

/// Есть ли у нас вот столько разных ресурсов
pub fn enough_resources(
    world: &mut World,
    required: HashMap<Resource, RealUnits>,
) -> bool {
    let mut result = true;
    for (res, amount) in required.iter() {
        let deposit = how_much_we_have(world, *res);
        if deposit < *amount {
            result = false;
            break;
        }
    }
    result
}

/// Списать ресурсы пачкой.
pub fn writeoff_bunch (
    world: &mut World,
    bunch: HashMap<Resource, RealUnits>
) -> Result<(),SamosborError> {
    if enough_resources(world, bunch.clone()) {
        for (res, amount) in bunch.iter() {
            let _ = writeoff(world, *res, *amount);
        }
        Ok (())
    } else {
        Err(SamosborError::NotEnoughResources)
    }
}

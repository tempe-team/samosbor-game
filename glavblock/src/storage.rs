use std::cmp::min;
use legion::*;


use crate::area::*;
use crate::core::*;
use crate::resources::*;

/// Стандартная вместимость контейнера(полки) в единицах объема
const CONTAINER_VOLUME:usize = 1000;

/// Сколько места занимает напольный контейнер
const CONTAINER_SIZE:usize = 5;

/// Тип ресурса. Текучий или твердый.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StorageType {
    Solid,
    Fluid,
}

/// Полка на стеллаже
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shelf ();

/// Чан (как контейнер для хранения, не как здание)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Barrel ();

/// Контейнер на полу
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Container ();

/// Сколько объемных единиц занято в контейнере
struct OccupiedVolume (pub usize);

/// Тип хранения ресурса
fn container_type (
    resource: Resource
) -> StorageType {
    match resource {
        BioRaw           => StorageType::Solid,
        Scrap            => StorageType::Solid,
        Concrete         => StorageType::Solid,
        IsoConcrente     => StorageType::Solid,
        TransparentSlime => StorageType::Fluid,
        BlackSlime       => StorageType::Fluid,
        BrownSlime       => StorageType::Fluid,
        RedSlime         => StorageType::Fluid,
        PinkSlime        => StorageType::Fluid,
        WhiteSlime       => StorageType::Fluid,
        Component        => StorageType::Solid,
        Reagent          => StorageType::Solid,
        Polymer          => StorageType::Solid,
        Ethanol          => StorageType::Fluid,
        Fuel             => StorageType::Fluid,
        Concentrat       => StorageType::Solid,
    }
}

/// Каждый контейнер имеет вместимость 1000 объемных единиц.
/// контейнер занимает 5 м. кв. пространства
/// Функция говорит сколько объемных единиц занимает одна
/// единица конкретного ресурса.
pub fn piece_size (
    resource: Resource,
) -> usize {
    match resource {
        BioRaw           => 100,
        Scrap            => 100,
        Concrete         => 1000,
        IsoConcrente     => 100,
        TransparentSlime => 50,
        BlackSlime       => 50,
        BrownSlime       => 50,
        RedSlime         => 50,
        PinkSlime        => 50,
        WhiteSlime       => 50,
        Component        => 100,
        Reagent          => 100,
        Polymer          => 10,
        Ethanol          => 1,
        Fuel             => 1,
        Concentrat       => 1,
    }
}

/// Хранить ресурс в чане.
/// Возвращает количество ресурса которое не удалось залить
/// на хранение. Если 0 - значит все залили.
fn store_in_barrels (
    world: &mut World,
    amount: usize,
    resource: Resource,
) -> usize {if amount == 0 {return 0} else {
    let mut query = <(&Barrel, &mut Option<Resource>, &mut OccupiedVolume)>::query();
    let free_barrels = query
        .iter_mut(world)
        .filter(|(_, res, _)| **res == None);

    let piece_volume = piece_size(resource);
    let mut not_deposited: usize = amount;
    for (_, mut res, occupied) in free_barrels {
        res = &mut Some(resource);
        // Объем, занимаемый данным количеством материалов.
        // Если материала слишком много - заполнить чан полностью
        // а для остатка искать следующий
        let occupied_volume = min(
            CONTAINER_VOLUME,
            not_deposited.clone() * piece_volume,
        );
        if occupied_volume == 0 { // Все распределено
            break
        } else { // Что-то осталось. Заходим на следующий виток.
            occupied.0 = occupied_volume;
            not_deposited -= occupied_volume / piece_volume;
        }
    }
    // Либо все распределили, либо бочки закончились и возвращаем сколько не разместили
    not_deposited
}}

/// Хранить ресурс на полке.
/// Возвращает количество ресурса которое не удалось положить
/// на хранение. Если 0 - значит все разложили.
fn store_on_shelves (
    world: &mut World,
    amount: usize,
    resource: Resource
) -> usize {if amount == 0 {return 0} else {
    let mut query = <(&Shelf, &mut Option<Resource>, &mut OccupiedVolume)>::query();
    let free_shelves = query
        .iter_mut(world)
        .filter(|(_, res, _)| **res == None);

    let piece_volume = piece_size(resource);
    let mut not_deposited: usize = amount;
    for (_, mut res, occupied) in free_shelves {
        res = &mut Some(resource);
        // Объем, занимаемый данным количеством материалов.
        // Если материала слишком много - заполнить полку полностью
        // а для остатка искать следующий
        let occupied_volume = min(
            CONTAINER_VOLUME,
            not_deposited.clone() * piece_volume,
        );
        if occupied_volume == 0 { // Все распределено
            break
        } else { // Что-то осталось. Заходим на следующий виток.
            occupied.0 = occupied_volume;
            not_deposited -= occupied_volume / piece_volume;
        }
    }
    // Либо все распределили, либо полки закончились и возвращаем сколько не разместили
    not_deposited
}}

/// Хранить ресурсы в ящиках на полу
/// Возвращает количество ресурса которое не вместилось
fn store_on_floor (
    world: &mut World,
    amount: usize,
    resource: Resource
) -> usize { if amount == 0 {return 0} else {
    let piece_volume = piece_size(resource);
    let mut not_deposited: usize = amount;
    while not_deposited > 0 {
        let mbroom = get_sufficent_room(
            world,
            CONTAINER_SIZE,
            AreaType::Party, // партийный склад!
        );
        match mbroom {
            None => break, // нет на складах места.
            Some (room) => {
                let occupied_volume = min(
                    CONTAINER_VOLUME,
                    not_deposited.clone() * piece_volume,
                );
                not_deposited -= occupied_volume
                    / piece_volume;
                world.push((
                    Container,
                    resource,
                    BelongsToRoom (room),
                    OccupiedVolume(occupied_volume),
                ));
            },
        }
    };
    not_deposited
}}

/// Положить ресурс на хранение.
/// Возвращает количество невместившегося ресурса.
pub fn put_resource(
    world: &mut World,
    resource: Resource,
    tier: Option<Tier>,
    amount: usize,
) -> usize {
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

/// Изъять ресурс, освободить пространство. Если ресурса нет в нужном количестве, вернуть ошибку.
pub fn take_resource (
    world: &mut World,
    resource: Resource,
    amount: usize,
) -> Result<(), ()> {
    // Проверить есть ли такой ресурс в нужном количестве в приципе
    // Если есть - найти наименее заполненный контейнер
    // и попытаться забрать оттуда требуемое количество

    // Если набралось требуемое количество - проверить, осталось ли что то в контейнере.
    // Если контейнер пуст, и лежит на полу - удалить его.
    // Вернуть успех.

    // Если после опустошения контейнера требуемого количества не набралось и если контейнер лежит на полу - уничтожить контейнер и перейти к следующему.  Иначе просто перейти к следующему.
    unimplemented!();
}

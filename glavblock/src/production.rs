use legion::*;
use crate::core::*;
use crate::area::*;
use crate::people::*;
use crate::resources::*;
use crate::storage::*;

use std::collections::HashMap;

/// Метка того, к какому стационарному объекту принадлежит эта штука
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BelongsToStationary (pub Entity);

/// Стационарные объекты
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stationary {
    None, // Отсутствие постройки. Заглушка для обозначения того,
    // что некоторые производственные задачи не требуют
    // стационарного оборудования

    // Производство и хранение
    BenchToolT1, // верстак
    BenchToolT2, // токарно-фрезерный
    BenchToolT3, // электроника, электротехника, 3d печать..
    FormatFurnace, // Печь-формовщик. Переплавка металлолома в пригодные материалы. Температурная обработка. Формовка плавких материалов в лист, прокат, трубу и прочее. Вулканизация. Изготовление концентрата.
    LabT1, // Абстрактное научное оборудование.
    LabT2, // Абстрактное научное оборудование. Крутое.
    LabT3, // Абстрактное научное оборудование. Супер крутое.
    Barrel, // Чаны, в которых проходят химические реакции или хранятся текучие ресурсы. Используются в комбинации с хим, биолабораторией или печью. Забирают некое сырье, некий реагент и через какое-то время отдают другое сырье или продукт.
    Rack, // Стеллаж. Ставится в складские помещения. Увеличивает вместимость последних.

    // Инфраструктура
    NeuroTerminal, // Терминал для связи с нейронетом. ЭВМ.
}

/// Гермкомплект. Инфраструктура конкертного помещения. Бывает T1, T2, T3.
pub struct Germ ();

/// Сколько единиц площади занимает стационарный объект
pub fn stationary_size (
    stationary: Stationary,
) -> AreaOccupied {
    match stationary  {
        Stationary::None => AreaOccupied(0),
        Stationary::BenchToolT1 => AreaOccupied(20),
        Stationary::BenchToolT2 => AreaOccupied(25),
        Stationary::BenchToolT3 => AreaOccupied(50),
        Stationary::FormatFurnace => AreaOccupied(50),
        Stationary::LabT1 => AreaOccupied(20),
        Stationary::LabT2 => AreaOccupied(40),
        Stationary::LabT3 => AreaOccupied(60),
        Stationary::Barrel => AreaOccupied(15),
        Stationary::Rack => AreaOccupied(5),
        Stationary::NeuroTerminal => AreaOccupied(5),
    }
}

/// Поставить герму + обустроить помещение
/// Только для инициализации, в процессе игры гермы будут строится стандартным для
/// построек способом(через системы с поглощением билдпавера).
pub fn install_germ  (
    world: &mut World,
    tier: Tier,
    purpose: AreaType,
) -> Entity {
    world.push((
        Germ(),
        tier.clone(),
        purpose,
        tier2germ_capacity(tier),
    ))
}

/// Вместимость гермы
fn tier2germ_capacity(tier: Tier) {
    match tier {
        Tier::NoTier => unreachable!(),
        Tier::T1 => AreaCapacity(30),
        Tier::T2 => AreaCapacity(150),
        Tier::T3 => AreaCapacity(500),
    };
}

/// Количество труда, которое должен затратить (затратил)
/// работник на выполнение задачи за одну смену
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct BuildPower(usize);

/// В ситуации, когда работнику высокого тира нечего делать
/// он может выполнять работу нижних тиров.
/// T2 работник может делать T1 задачи
/// лучше T1 работника.
/// Эта функция отвечает насколько конкретно лучше.
pub fn buildpower_downgrage_coef(
    worker_tier: Tier,
    target_tier: Tier,
    bp: BuildPower,
) -> BuildPower {
    match (worker_tier, target_tier) {
        (Tier::T1, Tier::T1) => bp,
        (Tier::T2, Tier::T2) => bp,
        (Tier::T3, Tier::T3) => bp,
        (Tier::T2, Tier::T1) => BuildPower(bp.0 * 2),
        (Tier::T3, Tier::T2) => BuildPower(bp.0 * 2),
        (Tier::T3, Tier::T1) => BuildPower(bp.0 * 4),
        _ => BuildPower(0),
    }
}

/// Сколько работы можно произвести на данном оборудовании
pub fn stationary_build_power(
    stationary: Stationary,
) -> BuildPower{
    match stationary {
        Stationary::None => BuildPower(0),
        Stationary::BenchToolT1 => BuildPower(10),
        Stationary::BenchToolT2 => BuildPower(20),
        Stationary::BenchToolT3 => BuildPower(40),
        Stationary::FormatFurnace => BuildPower(10),
        Stationary::LabT1 => BuildPower(10),
        Stationary::LabT2 => BuildPower(10),
        Stationary::LabT3 => BuildPower(10),
        Stationary::Barrel => BuildPower(10),
        Stationary::Rack => BuildPower(0),
        Stationary::NeuroTerminal => BuildPower(10),
    }
}

/// Что нужно по ресурсам чтобы поставить эту стационарку
pub fn stationary_required_resources (
    stationary: Stationary,
) -> HashMap<Resource, RealUnits> {
    match stationary {
        Stationary::None => HashMap::new(),
        Stationary::BenchToolT1 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::BenchToolT2 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::BenchToolT3 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::FormatFurnace => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::LabT1 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::LabT2 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::LabT3 => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::Barrel => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::Rack => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
        Stationary::NeuroTerminal => [
            (Resource::ScrapT1, RealUnits (1))
        ].iter().cloned().collect(),
    }
}

/// Метаданные по рабочей задаче
/// Где-то рядом с этой рабочей задачей в ECS лежит штука
/// которая собственно делается
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct TaskMeta {
    prof: Profession,
    tier: Tier, // Тир исполнителя
    bp: BuildPower,
    stationary: Stationary, // на каком оборудовании надо выполнять работу
    sci_spec: SciSpec,
}

/// Что надо по рабочим/оборудованию чтобы построить вот это здание
pub fn stationary_requirements(
    target: Stationary,
) -> Vec<TaskMeta> {
    match target {
        Stationary::BenchToolT1 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
                sci_spec: SciSpec::None,
            },
                                                        ],
        Stationary::BenchToolT2 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
                sci_spec: SciSpec::None,
            },
        ],
        Stationary::BenchToolT3 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
                sci_spec: SciSpec::None,
            },
        ],
        Stationary::FormatFurnace => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
                sci_spec: SciSpec::None,
            },
        ],
        Stationary::LabT1 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
                sci_spec: SciSpec::None,
            },
        ],
        Stationary::LabT2 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
                sci_spec: SciSpec::None,
            },
        ],
        Stationary::LabT3 => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
                sci_spec: SciSpec::None,
            },
        ],
        Stationary::Barrel => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
                sci_spec: SciSpec::None,
            },
        ],
        Stationary::Rack => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
                sci_spec: SciSpec::None,
            },
        ],
        Stationary::NeuroTerminal => vec![
            TaskMeta {
                prof: Profession::Worker,
                tier: Tier::T1,
                bp: BuildPower(10),
                stationary: Stationary::None,
                sci_spec: SciSpec::None,
            },
        ],
        Stationary::None => Vec::new (),
    }
}

/// Запустить постройку
pub fn start_build_task (
    world: &mut World,
    stationary: Stationary,
    room: Entity,
) -> Result<(), SamosborError> {
    let free_space = get_room_free_space(world, room);
    let required_space = stationary_size(stationary);
    if free_space < required_space.0 as i32 {
        Err(SamosborError::NotEnoughArea)
    } else {
        let required_resources = stationary_required_resources(stationary);
        let _ = writeoff_bunch(world, required_resources)?;
        world.push ((
            stationary,
            stationary_requirements(stationary),
            stationary_size (stationary),
            BelongsToRoom(room),
        ));
        Ok (())
    }
}

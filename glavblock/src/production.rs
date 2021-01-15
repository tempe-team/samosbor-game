use legion::*;
use crate::core::*;

/// Виды помещений
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Area {
    Living, // жилячейки
    Science, // лаборатории
    Military, // казармы
    Industrial, // технические и производственные помещения. терминалы, распределительные узлы, насосы, чаны, станки.
    Party, // склады, образовательные помещения, детские сады, школы, залы партсобраний
    Medical, // медпункты, операционные
}

// Характеристики помещений
/// Вместимость помещения
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AreaCapacity(pub usize);

/// Занятость помещения
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AreaOccupied(pub usize);

/// Метка принадлежности(установлено в, проживает в) какого-то объекта какому то помещению
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BelongsToArea(pub Entity);

/// Стационарные объекты
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stationary {
    // Производство и хранение
    BenchTool, // Станочное оборудование. T1 - верстак. Т2 - по механике. T3 - электроника, электротехника, 3d печать..
    FormatFurnace, // Печь-формовщик. Переплавка металлолома в пригодные материалы. Температурная обработка. Формовка плавких материалов в лист, прокат, трубу и прочее. Вулканизация. Изготовление концентрата.
    Lab, // Абстрактное научное оборудование. T1, T2, T3.
    Barrel, // Чаны, в которых проходят химические реакции или хранятся текучие ресурсы. Используются в комбинации с хим, биолабораторией или печью. Забирают некое сырье, некий реагент и через какое-то время отдают другое сырье или продукт.
    Rack, // Стеллаж. Ставится в складские помещения. Увеличивает вместимость последних.

    // Инфраструктура
    Germ, // Гермкомплект. Инфраструктура конкертного помещения. Бывает T1, T2, T3.
    NeuroTerminal, // Терминал для связи с нейронетом. ЭВМ.
    OperatingRoom, // Операционная
}

/// Поставить герму + обустроить помещение
/// Только для инициализации, в процессе игры гермы будут строится стандартным для
/// зданий способом(через системы с поглощеением билдпавера).
pub fn install_germ  (
    world: &mut World,
    tier: Tier,
    purpose: Area,
) -> Entity {
    world.push((
        Stationary::Germ,
        tier.clone(),
        purpose,
        tier2germ_capacity(tier),
        AreaOccupied(0),
    ))
}

/// Вместимость гермы
fn tier2germ_capacity(tier: Tier) {
    match tier {
        Tier::T1 => AreaCapacity(30),
        Tier::T2 => AreaCapacity(150),
        Tier::T3 => AreaCapacity(500),
    };
}

/// Запустить установку стационарного объекта
pub fn _start_build_task (
    _world: World,
    _kind: Stationary,
    _room: Entity,
) {
    unimplemented!();
}

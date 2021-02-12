use std::hash::Hash;
use legion::*;

use crate::core::*;
use crate::area::*;

/// Сколько места занимает человек
pub static COMRAD_RENTED_PLACE: usize = 10;

/// Какому отделу ликвидаторов принадлежит боец
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MilitaryDep {
    None, // Не военный
    OLPS, // Отдел Ликвидации Последствий Самосбора
    OBCU, // Отдел по Борьбе с Человеческими Угрозами
    OGB, // Отдел Государственной Безопасности
}

/// К какому НИИ тяготеет яйцеголовый
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SciSpec {
    None, // Не ученый
    Samosbor, // НИИ Самосбора и Последствий. Плесень, слизь, твари, абберации, патогены и прочее. Очистка материи от влияния самосбора.
    Nervonet, // НИИ Коммуникаций и Нервонета
    Culture, // НИИ Культуры и Оккультизма. Про фракции помимо партии.
    Space, // НИИ Пространства и Бетона
    Industry, // НИИ Материалов и Промышленности
    Weapon, // НИИ Вооружения
    Bio, // НИИ Регулярной Биологии. Изучает формы жизни не затронутые самосбором, либо стабильно существующие вопреки ему. В том числе - людей. Помимо людей - борщевик, лифтовых арахн, бетоноедов и прочее.
}

pub fn random_sci_spec () -> SciSpec {
    match d(1,7) {
        1 => SciSpec::Samosbor,
        2 => SciSpec::Nervonet,
        3 => SciSpec::Culture,
        4 => SciSpec::Space,
        5 => SciSpec::Industry,
        6 => SciSpec::Weapon,
        7 => SciSpec::Bio,
        _ => unreachable!(),
    }
}

/// Профессия
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Profession {
    Child, // Нет профессии
    Doctor, // Доктор
    Stalker, // Мусорщик
    Likvidator, // Ликвидатор
    Scientist, // Ученый
    Worker, // Работник производства
    Party, // Работники госаппарата. Сюда же входят материально ответственные кладовщики, СМИ, Преподаватели.
}

/// Защита дыхания, шмот налицо
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceSlot {
    Empty, // Нет снаряжения
    Respirator, // Респиратор - базовая защита дыхательных путей
    Inhaler, // Противогаз
    ExoskeletonZ, // Экзоскелет зарница занимает все слоты шмота
}

/// Защита головы
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadSlot {
    Empty, // Нет снаряжения
    Helmet, // Каска-хрущевка
    ExoskeletonZ, // Экзоскелет зарница занимает все слоты шмота
}

/// Защита корпуса
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TorsoSlot {
    Empty, // Нет снаряжения
    Clothes, // Кирзовые сапоги, тулуп, шапка. Базовая одежда с минимальными защитными свойствами. Легкие.
    Robe, // Халаты ученого или доктора
    ExoskeletonU, // Экзоскелет 1-у. Базовый экзоскелет увеличивающий силу, со слабыми защитными свойствами
    ExoskeletonZ, // Экзоскелет «Зарница-2». Бронекостюм с системой жизнеобеспечения.
    HogweedSuit, // Бронекостюм Борщевик (средняя защита от проникающих ранений, хорошая от химических воздействий), тяжелый
    ConcreteSuit, // Бронекостюм Бетон (хорошая защита от проникающих ранений, слабая от физических воздействий), тяжелый
    Bulletproof, // Бронежилет 3-г. Средняя защита от пуль, слабая от химии, легкий.
}

/// Защита ног
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegsSlot {
    Empty,
    TarpaulinBoots, // Кирзачи
    HogweedSuit, // нижняя часть бронекостюма Борщевик (озк, все такое)
    ExoskeletonU, // Экзоскелет У-1 занимает слоты ног и торса
    ExoskeletonZ, // Экзоскелет зарница занимает все слоты шмота
}

/// Инструмент/Вооружение
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmsSlot {
    Empty, // Нет снаряжения

    // Военные
    BattleRake, // Боевые грабли. Без граблистов нет ликвидации - слизь и плесень соскребать надо. И в чаны собирать.
    Flamethrower, // Огнемет.. ну или сжигать. Но тогда ресурсы не пополнятся.
    Granite, // Дробовик "Гранит"
    PG6, // Пенобетонная граната
    Grinder, // Гравижернов
    RPL, // Ручной пулемет Лёшкинского
    AE, // Автомат Ералашникова
    PPS, // Пистолет - пулемет Слизнева
    PM, // Пистолет ПМ
    Spark, // Сварочный аппарат Искра

    // Рабочие
    WorkToolSet,

    // Наука
    ScienceToolSet,
}


/// Заспавнить колониста в конкретную комнату
pub fn spawn_comrad(
    world: &mut World,
    prof: Profession,
    tier: Tier,
    arms: ArmsSlot,
    face: FaceSlot,
    head: HeadSlot,
    torso: TorsoSlot,
    mdep: MilitaryDep,
    nii: SciSpec,
    room: Entity,
) -> Entity {
    let entity = world.push ((
        prof,
        tier,
        arms,
        face,
        head,
        torso,
        BelongsToRoom(room),
    ));
    let mut entry = world.entry(entity).unwrap();
    entry.add_component(AreaOccupied(COMRAD_RENTED_PLACE));
    entry.add_component(mdep);
    entry.add_component(nii);
    entity
}

/// Отряд ликвидаторов ОЛПС по стандарту №1-Ж
pub fn spawn_1_g (
    world: &mut World,
    room: Entity,
) {
    // Сержант с огнеметом
    spawn_comrad(
        world,
        Profession::Likvidator,
        Tier::T2,
        ArmsSlot::Flamethrower,
        FaceSlot::Inhaler,
        HeadSlot::Helmet,
        TorsoSlot::HogweedSuit,
        MilitaryDep::OLPS,
        SciSpec::None,
        room,
    );

    // Пятеро огнеметчиков
    for _ in 0..5 {
        spawn_comrad(
            world,
            Profession::Likvidator,
            Tier::T1,
            ArmsSlot::Flamethrower,
            FaceSlot::Inhaler,
            HeadSlot::Helmet,
            TorsoSlot::HogweedSuit,
            MilitaryDep::OLPS,
            SciSpec::None,
            room,
        );
    };

    // Четверо граблистов
    for _ in 0..4 {
        spawn_comrad(
            world,
            Profession::Likvidator,
            Tier::T1,
            ArmsSlot::BattleRake,
            FaceSlot::Inhaler,
            HeadSlot::Helmet,
            TorsoSlot::HogweedSuit,
            MilitaryDep::OLPS,
            SciSpec::None,
            room,
        );
    };

    // Наряд с искрой
    for _ in 0..2 {
        spawn_comrad(
            world,
            Profession::Likvidator,
            Tier::T1,
            ArmsSlot::Spark,
            FaceSlot::Inhaler,
            HeadSlot::Helmet,
            TorsoSlot::HogweedSuit,
            MilitaryDep::OLPS,
            SciSpec::None,
            room,
        );
    };

    // Двое с Гранитом
    for _ in 0..2 {
        spawn_comrad(
            world,
            Profession::Likvidator,
            Tier::T1,
            ArmsSlot::Granite,
            FaceSlot::Inhaler,
            HeadSlot::Helmet,
            TorsoSlot::HogweedSuit,
            MilitaryDep::OLPS,
            SciSpec::None,
            room,
        );
    };
}

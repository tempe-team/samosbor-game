use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;

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

/// Сытость. Согласно этой статье
/// https://pikabu.ru/story/kak_dolgo_chelovek_mozhet_prozhit_bez_edyi_3570894
/// средний человек живет без еды примерно 20 дней.
/// Один день - 10 единиц сытости.
/// сытость в районе 200 - это хорошо поевший человек.
/// Больше 200 - ожирение и прочие дебафы.
/// 1 съетая пачка концентрата добавляет 11 единиц насыщения если сытость меньше 190. Если больше 190 - 10.
/// На 100 начинается граница голодания с дебафами настроения.
/// На 0 голодная смерть.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Satiety(pub u16);

/// Настроение комрада.
/// Больше 10 быть не должно. 10 - счастлив.
/// 5, 6 - нейтрал
/// 0 - тотально несчастлив.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mood(pub u8);

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
    NoProf, // Нет профессии
    Stalker, // Мусорщик
    Likvidator, // Ликвидатор
    Scientist, // Ученый
    Worker, // Работник производства
    Party, // Работники госаппарата. Сюда же входят материально ответственные кладовщики, СМИ, Преподаватели.
}

impl fmt::Display for Profession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            NoProf => "Тунеядец".to_string(),
            Stalker => "Мусорщик".to_string(),
            Likvidator => "Ликвидатор".to_string(),
            Scientist => "Ученый".to_string(),
            Worker => "Рабочий".to_string(),
            Party => "Партийный функционер".to_string(),
        };
        write!(f, "{}", name)
    }
}

/// Заспавнить колониста в конкретную комнату
pub fn spawn_comrad(
    world: &mut World,
    prof: Profession,
    tier: Tier,
    mdep: MilitaryDep,
    nii: SciSpec,
    room: Entity,
) -> Entity {
    let entity = world.push ((
        prof,
        tier,
        BelongsToRoom(room),
        AreaOccupied(COMRAD_RENTED_PLACE),
        mdep,
        nii,
        Satiety(100),
        Mood(5),
    ));
    entity
}

/// Сколько у нас людей по профессиям
pub fn people_by_profession(
    world: &mut World,
) -> HashMap<Profession, usize> {
    let mut result = HashMap::new();
    let mut query = <&Profession>::query();
    for prof in query.iter(world) {
        let val = result
            .entry(*prof)
            .or_insert(0);
        *val += 1;
    }
    result
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
            MilitaryDep::OLPS,
            SciSpec::None,
            room,
        );
    };
}

/// Сумма всего настроения в блоке
pub fn block_mood(
    world: &mut World,
) -> usize {
    let mut result = 0;
    let mut query = <&Mood>::query();
    for Mood(m) in query.iter(world) {
        result += *m as usize
    };
    result
}

/// Насколько накормлены люди
pub fn block_satiety(
    world: &mut World,
) -> Satiety {
    let mut result = 0;
    let mut query = <&Satiety>::query();
    for Satiety(m) in query.iter(world) {
        result += m
    };
    Satiety(result)
}

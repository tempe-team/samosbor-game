use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::min;

use legion::*;
use legion::world::*;

use crate::core::*;
use crate::people::*;
use crate::production::*;
use crate::storage::*;
use crate::resources::*;

pub type BuildPowerPool = HashMap<Profession,HashMap<Tier, BuildPower>>;

pub fn turn(
    world: &mut World,
    resources: &mut Resources,
) {
    calc_buildpower(world, resources);
    process_tasks(world, resources);
    clean_up_completed_tasks(world, resources);
    setup_completed_stationaries(world, resources);
    hunger_tick(world, resources);
    consume_concentrat(world, resources);
}

/// Сформировать пул билдпавера
fn calc_buildpower(
    world: &mut World,
    resources: &mut Resources,
) {
    let mut buildpower_pool = resources
        .get_mut::<BuildPowerPool>()
        .unwrap();
    let mut people_query = <(
        &Profession,
        &Tier,
    )>::query();
    for (prof, tier) in people_query.iter(world) {
        let human_bp = tier2comrad_buildpower(tier.clone());
        let by_tier_hm = buildpower_pool
            .entry(*prof)
            .or_insert(HashMap::new());

        let bp = by_tier_hm
            .entry(*tier)
            .or_insert(BuildPower(0));
        *bp += human_bp;
    }
}

/// Распределить все очки работы по заданиям
/// TODO: Наивная реализация. T3 инженеры на T3
/// станках тоже должны уметь делать T1 задания, причем
/// более эффективно чем T1 работяги на T1 станках.
/// Надо писать правила деградации.
pub fn process_tasks(
    world: &mut World,
    resources: &mut Resources,
) {
    let mut buildpower_pool = resources
        .get_mut::<BuildPowerPool>()
        .unwrap();
    let mut stationary_query = <(
        &Stationary,
        &StationaryStatus,
    )>::query();
    let mut stationaries:HashMap<Stationary, BuildPower> =
        HashMap::new();

    for (stat, status) in stationary_query.iter(world) {
        if (*status) == StationaryStatus::Ready {
            let bp = stationary_build_power(*stat);
            let bp_for_update = stationaries
                .entry(*stat)
                .or_insert(BuildPower(0));
            *bp_for_update += bp;
        }
    };
    let mut tasks_query = <(
        &TaskPriority,
        &mut TaskMeta,
    )>::query();
    let mut tasks: Vec<(
        &TaskPriority,
        &mut TaskMeta,
    )> = tasks_query
        .iter_mut(world)
        .collect();
    tasks.sort_by(|(p1, _), (p2, _)|(**p1).cmp(*p2));
    for (_priority, task) in tasks.iter_mut() {
        if let Some(stationary_bp) = stationaries.get_mut(&task.stationary) {
            if let Some(by_tier) = buildpower_pool.get_mut(&task.prof) {
                if let Some(human_bp) = by_tier.get_mut(&task.tier){
                    let lesser_bp = min(
                        human_bp.clone(),
                        min(
                            stationary_bp.clone(),
                            task.bp.clone(),
                        )
                    );
                    // Вот этот человек сделал
                    *human_bp -= lesser_bp;
                    // ...на на вот этом станке
                    *stationary_bp -= lesser_bp;
                    // ...вот столько работы
                    task.bp -= lesser_bp;
                }
            }
        }
    }
}

/// Убрать выполненные таски
pub fn clean_up_completed_tasks(
    world: &mut World,
    _resources: &mut Resources,
) {
    let mut to_delete:HashSet<Entity> = HashSet::new();
    let mut query = <(&Entity, &TaskMeta)>::query();
    for (entity, task) in query.iter(world) {
        if task.bp == BuildPower (0) {
            // Весь требуемый билдпавер влит в эту задачу
            // Задача завершена.
            to_delete.insert(entity.clone());
        };
    };
    for entity in to_delete.iter () {
        world.remove(*entity);
    };
}

/// Глянуть если есть завершенные задания по строительству
/// стационарных объектов
/// Если есть - ввести в эксплуатацию.
pub fn setup_completed_stationaries(
    world: &mut World,
    _resources: &mut Resources,
) {
    // Стационарки по которым есть незакрытые таски.
    // Предполагается что завершенные таски удалены предыдущей системой.
    let mut under_construction : HashSet<Entity>  = HashSet::new();
    let mut under_construction_q = <&BelongsToStationary>::query()
        .filter(component::<TaskMeta>());

    for BelongsToStationary(entity) in under_construction_q.iter (world) {
        under_construction.insert(*entity);
    };
    let mut stats_query = <(&Entity, &mut StationaryStatus)>::query();
    // стационарки которые строятся и не введены в эксплуатацию
    for (entity, status) in stats_query
        .iter_mut(world)
        .filter(
            |(_, status)|
            **status == StationaryStatus::Constructing)
    {
        // статус стационарного объекта - конструируется.
        // но по нему нет активных задач.
        // И на самом деле это означает что конструкция завершена.
        if let Some(_) = under_construction.get(entity) {
            *status = StationaryStatus::Ready;
        }
    }
}

/// Голод
pub fn hunger_tick(
    world: &mut World,
    _resources: &mut Resources,
) {
    let mut died_by_hunger: Vec<Entity> = Vec::new();
    let mut query = <(
        &Entity,
        &mut Satiety,
        &mut Mood,
    )>::query();
    for (entity, sat, mood) in query.iter_mut(world) {
        sat.0 -= 10;
        if sat.0 < 10 {
            died_by_hunger.push(*entity);
        }
        // ниже ста - голод - минус настроение
        if sat.0 < 100 {
            mood.0 -= 1;
        }
    }
    for e in died_by_hunger.iter() {
        world.remove(*e);
    }
}

/// Люди едят концентрат
pub fn consume_concentrat(
    world: &mut World,
    _resources: &mut Resources,
) {
    // сколько есть на складе
    let mut t1_conc_amount = how_much_we_have(
        world,
        Resource::ConcentratT1,
    );
    // Сколько выдано
    let mut t1_conc_writeroff = 0;
    // имеет настроение = человек.
    // да, знаю, зашибись признак.
    let mut query = <(
        &mut Mood,
        &mut Satiety,
    )>::query();

    for (mood, sat) in query.iter_mut(world){
        if t1_conc_amount.0 <= 0 {
            // Не дали пожрать. Настроение
            // от такого ухудшается.
            mood.0 -= 1;
        } else {
            t1_conc_amount.0 -= 1;
            t1_conc_writeroff += 1;
            mood.0 += 1;
            sat.0 += 10;
        }
    }
    writeoff(
        world,
        Resource::ConcentratT1,
        RealUnits(t1_conc_writeroff),
    );
}

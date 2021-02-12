use legion::*;
use legion::world::*;
use crate::core::*;
use crate::people::*;
use crate::production::*;

use std::collections::HashMap;

pub type BuildPowerPool = HashMap<Profession,HashMap<Tier, BuildPower>>;

#[system]
#[read_component(Profession)]
#[read_component(Tier)]
/// Посчитать всем человекам их очки работы
pub fn spread_buildpower(
    world: &mut SubWorld,
    #[resource] buildpower_pool: &mut BuildPowerPool,
) {
    let mut people_query = <(
        &Profession,
        &Tier,
    )>::query();
    for (prof, tier) in people_query.iter(world) {
        let human_bp = tier2comrad_buildpower(tier.clone());
        let mut by_tier_hm = buildpower_pool
            .entry(*prof)
            .or_insert(HashMap::new());

        let mut bp = by_tier_hm
            .entry(*tier)
            .or_insert(BuildPower(0));
        *bp += human_bp;
    }
}

#[system]
#[read_component(TaskPriority)]
#[read_component(TaskMeta)]
#[write_component(TaskMeta)]
/// Распределить все очки работы по заданиям
pub fn process_tasks(
    world: &mut SubWorld,
    #[resource] buildpower_pool: &mut BuildPowerPool,
) {
    let mut tasks_query = <(
        &TaskPriority,
        &mut TaskMeta,
    )>::query();
    let mut tasks: Vec<(&TaskPriority, &mut TaskMeta)> = tasks_query
        .iter_mut(world)
        .collect();
    tasks.sort_by(|(p1, _), (p2, _)|(**p1).cmp(*p2));
    unimplemented!()
}

#[system]
#[write_component(TaskMeta)]
/// Глянуть если есть завершенные задания по строительству
/// стационарных объектов
/// Если есть - ввести в эксплуатацию.
pub fn setup_completed_stationaries(
    world: &mut SubWorld
) {
    unimplemented!();
}


#[system]
#[write_component(TaskMeta)]
/// Глянуть если есть завершенные задания по
/// производству шмота
/// Если есть - положить на хранение
pub fn setup_completed_equipment(
    world: &mut SubWorld
) {
    unimplemented!();
}


#[system]
#[write_component(TaskMeta)]
/// Глянуть если есть завершенные задания по
/// производству ресурсов. Если есть - положить на хранение.
pub fn setup_completed_resources(
    world: &mut SubWorld
) {
    unimplemented!();
}


#[system]
pub fn consume_concentrat () {
    unimplemented!();
}


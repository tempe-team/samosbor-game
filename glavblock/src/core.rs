use std::hash::Hash;
use rand::thread_rng;
use rand::Rng;

/// Бросить кубы
pub fn d(rolls:u8, sides:u8) -> usize {
    if sides < 1 || rolls < 1 {
        0
    } else {
        let mut rng = thread_rng();
        let mut result = 0;
        for _ in 0..rolls {
            result += rng.gen_range(0..sides) as usize
        }
        result
    }
}

pub enum SamosborError {
    NoEmptyArea,
    NotEnoughArea,
    NotEnoughResources,
}

/// Уровень(изделия, опыта, ресурса и тп)
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Tier {
    NoTier, // уникальные штуки
    T1,
    T2,
    T3,
}

pub enum _Language {
    RU,
    EN,
}

use legion::*;
use crate::core::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Resource {
    BioRaw, // биологическое сырье. Бывает загрязненное(т1), чистое (т2) и очищенное (т3)
    Scrap, // Металлолом. Т1 - чермет, Т2 - Цветмет, Т3 - редкие металлы
    Concrete, // Бетонная крошка
    IsoConcrente, // Изобетон. Артефактный ресурс.

    TransparentSlime, // Прозрачная слизь
    BlackSlime, // Черная слизь
    BrownSlime, // Коричневая слизь
    RedSlime,   // Красная слизь
    PinkSlime,  // Розовая слизь
    WhiteSlime, // Белая слизь. Артефактный ресурс.

    Component, // Компонент. T1 - механика, Т2 - электроника, Т3 - Артефакты

    Reagent, // Реагенты, нужные для крафта. T1 - экоцид, разрушение. T2 - компониум, объединение. T3 - сталий, изменение.

    Polymer, // Полимер. Он же резина. Он же синтетическая ткань. T1, T2, T3.
    Ethanol, // Этанол. Может использоваться для бухла, в медицине и для техпроцессов.
    Fuel, // Универсальное топливо. Используется при изготовлении боеприпасов. Так же работает как растворитель в техпроцессах. Ядовито.
    Concentrat, // концентрат. T1 - белый, T2 - черный, T3 - красный
}

/// Тип ресурса. Текучий или твердый.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageType {
    Solid,
    Fluid,
}

/// Тип хранения ресурса
pub fn container_type (
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
/// Функция говорит сколько объемных единиц занимает одна
/// единица конкретного ресурса.
pub fn piece_size (
    resource: Resource
) -> usize {
    match resource {
        BioRaw           => 200,
        Scrap            => 1000,
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

pub fn init_container(
    world: &mut World,
    stationary: Option<Entity>,
) -> Result<Entity, SamosborError>{
    // Есть ли вообще
    unimplemented!()
}

/// Положить ресурс на хранение
pub fn put_resource(
    world: &mut World,
    resource: Resource,
    amount: usize,
) -> Result<(), ()> {
    unimplemented!();
    // Взять контейнер с таким же ресурсом
    // Или контейнер без ресурса
    // Или создать новый контейнер
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

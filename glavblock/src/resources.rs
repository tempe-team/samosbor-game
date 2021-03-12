use std::fmt;
use std::hash::Hash;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Resource {
    BioRawT1, // загрязненное биологическое сырье.
    BioRawT2, // чистое биологическое сырье.
    BioRawT3, // очищенное биологическое сырье.
    ScrapT1, // Лом черных металлов
    ScrapT2, // Лом цветных металлов
    ScrapT3, // Лом редких металлов
    Concrete, // Бетонная крошка
    IsoConcrente, // Изобетон. Артефактный ресурс.

    TransparentSlime, // Прозрачная слизь
    BlackSlime, // Черная слизь
    BrownSlime, // Коричневая слизь
    RedSlime,   // Красная слизь
    PinkSlime,  // Розовая слизь
    WhiteSlime, // Белая слизь. Артефактный ресурс.

    ComponentT1, // механический компонент
    ComponentT2, // электронный компонент
    ComponentT3, // артефактный компонент

    ReagentT1, // экоцид - реактив разрушения
    ReagentT2, // компониум - реактив объединения
    ReagentT3, // сталий - реактив изменения.

    PolymerT1, // Синтетическая ткань
    PolymerT2, // пластик
    PolymerT3, // супер пластик

    ConcentratT1, // белый пищевой концентрат
    ConcentratT2, // черный пищевой концентрат
    ConcentratT3, // красный пищевой концентрат
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            NoProf => "Тунеядец".to_string(),
            BioRawT1 => "Загрязненное биосырье".to_string(),
            BioRawT2 => "Биосырье".to_string(),
            BioRawT3 => "Очищенное биосырье".to_string(),
            ScrapT1 => "Чермет".to_string(),
            ScrapT2 => "Цветмет".to_string(),
            ScrapT3 => "Редкие металлы".to_string(),
            TransparentSlime => "Прозрачная слизь".to_string(),
            BlackSlime => "Черная слизь".to_string(),
            BrownSlime => "Коричневая слизь".to_string(),
            RedSlime   => "Красная слизь".to_string(),
            PinkSlime  => "Розовая слизь".to_string(),
            WhiteSlime => "Белая слизь".to_string(),

            ComponentT1 => "Механический компонент".to_string(),
            ComponentT2 => "Электронный компонент".to_string(),
            ComponentT3 => "Суперкомпонент".to_string(),

            ReagentT1 => "Экоцид".to_string(),
            ReagentT2 => "Компониум".to_string(),
            ReagentT3 => "Сталий".to_string(),

            PolymerT1 => "Синтетическая ткань".to_string(),
            PolymerT2 => "Пластик".to_string(),
            PolymerT3 => "Суперпластик".to_string(),

            ConcentratT1 => "Белый концентрат".to_string(),
            ConcentratT2 => "Черный концентрат".to_string(),
            ConcentratT3 => "Красный концентрат".to_string(),
        };
        write!(f, "{}", name)
    }
}

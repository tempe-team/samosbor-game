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

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
    Concentrat, // концентрат. T1 - белый, T2 - черный, T3 - красный
}

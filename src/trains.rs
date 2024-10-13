use bevy::color::{palettes::css, Color};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TrainColor {
    Brown,
    Red,
    Blue,
    Yellow,
    Purple,
    Green,
    Orange,
}

impl From<TrainColor> for Color {
    fn from(value: TrainColor) -> Self {
        match value {
            TrainColor::Brown => Color::Srgba(css::BROWN),
            TrainColor::Red => Color::Srgba(css::RED),
            TrainColor::Blue => Color::Srgba(css::BLUE),
            TrainColor::Yellow => Color::Srgba(css::YELLOW),
            TrainColor::Purple => Color::Srgba(css::PURPLE),
            TrainColor::Green => Color::Srgba(css::GREEN),
            TrainColor::Orange => Color::Srgba(css::ORANGE),
        }
    }
}

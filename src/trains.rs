use bevy::color::Color;

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
            TrainColor::Brown => Color::srgb(0.471, 0.333, 0.231),
            TrainColor::Blue => Color::srgb(0.165, 0.314, 0.773),
            TrainColor::Red => Color::srgb(0.733, 0.153, 0.122),
            TrainColor::Yellow => Color::srgb(0.918, 0.918, 0.396),
            TrainColor::Orange => Color::srgb(0.914, 0.624, 0.220),
            TrainColor::Green => Color::srgb(0.376, 0.788, 0.231),
            TrainColor::Purple => Color::srgb(0.631, 0.125, 0.773),
        }
    }
}

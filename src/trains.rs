use bevy::color::Color;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

impl TrainColor {
    pub fn mix_with(self: TrainColor, other: TrainColor) -> TrainColor {
        if self == other {
            return other;
        }

        if (self == TrainColor::Blue && other == TrainColor::Red)
            || (self == TrainColor::Red && other == TrainColor::Blue)
        {
            return TrainColor::Purple;
        }
        if (self == TrainColor::Yellow && other == TrainColor::Red)
            || (self == TrainColor::Red && other == TrainColor::Yellow)
        {
            return TrainColor::Orange;
        }
        if (self == TrainColor::Yellow && other == TrainColor::Blue)
            || (self == TrainColor::Blue && other == TrainColor::Yellow)
        {
            return TrainColor::Green;
        }

        TrainColor::Brown
    }
    pub fn mix_many(trains: Vec<TrainColor>) -> TrainColor {
        match trains.len() {
            1 => trains[0],
            2 => trains[1].mix_with(trains[0]),
            _ => {
                for i in 0..(trains.len() - 1) {
                    if trains[i] != trains[i + 1] {
                        return TrainColor::Brown;
                    }
                }
                trains[0]
            }
        }
    }
}

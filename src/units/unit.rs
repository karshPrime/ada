use gpiod::Chip;

pub trait Component {
    fn init(chip: &Chip) -> Self;
    fn update(&mut self, counter: &u32);
}

#[derive(Debug, Copy, Clone)]
pub enum Pace {
    Fast,
    Medium,
    Slow,
    Still,
}

pub fn pace_value(pace: Pace) -> u32 {
    match pace {
        Pace::Fast => 100,
        Pace::Medium => 75,
        Pace::Slow => 50,
        Pace::Still => 0,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Pulse {
    pub pace: u32,
    pub count: u8,
}


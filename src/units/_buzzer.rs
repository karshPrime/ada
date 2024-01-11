use gpiod;
use super::unit;

pub struct Buzzer {
    sleep: u32,
}

impl unit::Component for Buzzer {
}

use gpiod;
use super::unit;

pub struct Led {
    sleep: u32,
}

impl unit::Component for Led {
}

use gpiod;
use super::unit;

pub struct Led {
    line: gpiod::Lines<gpiod::Input>,
    sleep: u32,
}

impl unit::Component for Led {
    // Initialize and return a vector of LEDs
    fn init(chip: &gpiod::Chip) -> Led {
        unimplemented!()
    }

    fn update(&self, counter: &u32) {
        //
    }
}

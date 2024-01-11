use gpiod;
use super::unit;

pub struct Led {
    sleep: u32,
}

impl unit::Component for Led {
    // Initialize and return a vector of LEDs
    fn init(chip: &gpiod::Chip) -> Vec<Box<Self>> {
        unimplemented!()
    }

    fn update(leds: &[Box<Self>], counter: &u32) -> () {
        for led in leds {
            // Update logic for led
        }
    }

    fn free(leds: &[Box<Self>]) -> () {
        for led in leds {
            // Free resources for led
        }
    }
}

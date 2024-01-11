use gpiod;
use super::unit;

pub struct Button {
    sleep: u32,
}

impl unit::Component for Button {
    // Initialize and return a vector of buttons
    fn init(chip: &gpiod::Chip) -> Vec<Box<Self>> {
        unimplemented!()
    }

    fn update(buttons: &[Box<Self>], counter: &u32) -> () {
        for button in buttons {
            // Update logic for button
        }
    }

    fn free(buttons: &[Box<Self>]) -> () {
        for button in buttons {
            // Free resources for button
        }
    }
}

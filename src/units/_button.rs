use gpiod;
use super::unit;

const PIN_COUNT: usize = 4;

pub struct Button {
    line: gpiod::Lines<gpiod::Input>,
    sleep: [u32; PIN_COUNT],
}

impl unit::Component for Button {
    // Initialize and return a vector of buttons
    fn init(chip: &gpiod::Chip) -> Vec<Box<Self>> {
        unimplemented!()
    fn init(chip: &gpiod::Chip) -> Button {
        return Button {line: connection_line, sleep: sleep_status};
    }

    fn update(&self, counter: &u32) {
        if 0 == self.sleep[0] {
            println!("{counter}");
        }
    }

    }
}


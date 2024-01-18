use gpiod;
use super::unit;

const PIN_COUNT: usize = 3;

pub struct Led {
    line: gpiod::Lines<gpiod::Output>,
    sleep: [u32; PIN_COUNT],
    blink: [unit::Pulse; PIN_COUNT],
}

impl unit::Component for Led {
    // Initialize and return a vector of LEDs
    fn init(chip: &gpiod::Chip) -> Led {
        let pins: [u32; PIN_COUNT] = [
            26, // 0  indicate power stability
            21, // 1  indicate internet connectivity
            23  // 2  indicate shift status
            // ...add more LEDs here
        ];

        let sleep_status: [u32; PIN_COUNT] = Default::default(); 
        let blink_status: [unit::Pulse; PIN_COUNT] = [
            unit::Pulse {pace: 0, count: 0,}; 
            PIN_COUNT
        ];

        let options = gpiod::Options::output(pins)
            .consumer("LEDs");

        let connection_line = chip
            .request_lines(options)
            .expect("failed to Initialize leds");

        return Led { 
            line: connection_line,
            sleep: sleep_status,
            blink: blink_status
        };
    }

    fn update(&mut self, counter: &u32) {
    }

    }
}


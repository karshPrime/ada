use gpiod::Chip;

mod trigger;
mod units;

use units::{unit::Component, Led, Button, Buzzer};

fn main() {
    let mut counter: u32 = 1;
    let mut run: bool = true;

    // initialise all components
    let chip = Chip::new("gpiochip0").expect("Failed to open GPIO chip");
    let leds = Led::init(&chip);
    let buttons = Button::init(&chip);
    let buzzers = Buzzer::init(&chip);
    
    while run {
        // update
        Led::update(&leds, &counter);
        Button::update(&buttons, &counter);
        Buzzer::update(&buzzers, &counter);

        // actions
        trigger::auto();
        trigger::call();

        // counter update
        trigger::update_counter(&mut counter);

        // if program must keep running
        run = trigger::keep_running();
    }

    // free all components
    Led::free(&leds);
    Button::free(&buttons);
    Buzzer::free(&buzzers);
}

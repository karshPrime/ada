mod log;
mod units;
mod trigger;
mod counter;

use gpiod::Chip;
use units::{unit::Component, Led, Button, Buzzer};

fn main() {
    let mut counter: u32  = 1;
    let mut run    : bool = true;
    let mut shift  : bool = false;

    // initialise all components
    let chip = Chip::new("gpiochip0")
        .expect("Failed to open GPIO chip");
    
    let leds = Led::init(&chip);
    let buttons = Button::init(&chip);
    let buzzers = Buzzer::init(&chip);
    
    while run {
        // update
        leds.update(&counter);
        buttons.update(&counter);
        buzzers.update(&counter);

        // actions
        let cmd_call = buttons.call();
        trigger::auto(&counter, &leds, &buzzers);
        trigger::call(&mut shift, &cmd_call);

        // counter update
        counter::update(&mut counter);

        // if program must keep running
        run = trigger::keep_running(&buttons);
    }
}


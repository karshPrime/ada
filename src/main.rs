use gpiod::Chip;
mod trigger;
mod units;

use units::{unit::Component, Led, Button, Buzzer};

fn main() {
    let mut counter: u32 = 1;
    let mut run: bool = true;

    // initialise all components
    
    while run {
        // update

        // actions
        trigger::auto();
        trigger::call();

        // counter update
        trigger::update_counter(&mut counter);

        // if program must keep running
        run = trigger::keep_running();
    }

    // free all components
}

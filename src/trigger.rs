use crate::units;
use units::{Led, Button, Buzzer};

//
pub fn auto(counter: &u32, leds: &Led, buzzers: &Buzzer) {
    if 1 == *counter {
        // change led status
    }
    unimplemented!();
}

//
pub fn call(shift: &mut bool, cmd: &u16) {
    let status: u16 = 10 * (*shift as u16) + *cmd;

    match status {
        10 => {
            *shift = false;
            // turn led off
        },

        11 => {
            *shift = true;
            // turn led on
        },

        // ========================= //
        _ => {}
    }
}

//
pub fn keep_running(buttons: &Button) -> bool {
    unimplemented!();
}


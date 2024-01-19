// counter.rs

// update the counter 
pub fn update(counter: &mut u32)  {
    *counter = 1 + (*counter * (*counter == u32::MAX) as u32);
}

// wrap numbers
pub fn next(counter: &u32, span: &u32) -> u32 {
    return 1 + (counter + span) % u32::MAX;
}


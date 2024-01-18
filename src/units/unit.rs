use gpiod::Chip;

pub trait Component {
    fn init(chip: &Chip) -> Self;
    fn update(&mut self, counter: &u32);
}

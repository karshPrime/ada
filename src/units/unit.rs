use gpiod::Chip;

pub trait Component {
    fn init(chip: &Chip) -> Self;
    fn update(&self, counter: &u32);
}

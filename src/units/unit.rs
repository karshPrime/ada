use gpiod::Chip;

pub trait Component {
    fn init(chip: &Chip) -> Vec<Box<Self>>;
    fn update(units: &[Box<Self>], counter: &u32);
    fn free(units: &[Box<Self>]);
}

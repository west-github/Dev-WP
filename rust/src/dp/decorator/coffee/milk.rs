use super::Coffee;

pub struct Milk<T>
where
    T: Coffee,
{
    quantity: i32,
    coffee: T,
}

impl<T> Milk<T>
where
    T: Coffee,
{
    pub fn new(quantity: i32, coffee: T) -> Self {
        Self { quantity, coffee }
    }
}

impl<T> Coffee for Milk<T>
where
    T: Coffee,
{
    fn description(&self) -> String {
        format!("{} with Milk", self.coffee.description())
    }

    fn calc_price(&self) -> i32 {
        30 + self.coffee.calc_price()
    }
}

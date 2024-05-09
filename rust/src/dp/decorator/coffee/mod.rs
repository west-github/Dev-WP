mod decaf;
mod milk;

const TAX: i32 = 20;

pub(self) trait Coffee {
    fn description(&self) -> String;

    fn calc_price(&self) -> i32;
}

#[cfg(test)]
mod decorator_coffee_tests {
    use super::{decaf::Decaf, milk::Milk, Coffee};

    #[test]
    fn test_one() {
        let milk = Milk::new(10, Decaf::new(10));

        println!("{} - {}", milk.description(), milk.calc_price());
    }
}

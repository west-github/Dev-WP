use super::{Coffee, TAX};

pub struct Decaf {
    quantity: i32,
}

impl Decaf {
    pub fn new(quantity: i32) -> Self {
        Self { quantity }
    }
}

impl Coffee for Decaf {
    fn description(&self) -> String {
        String::from("Decaf")
    }

    fn calc_price(&self) -> i32 {
        self.quantity * 10 + TAX
    }
}

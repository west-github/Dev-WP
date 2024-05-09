use super::Summary;

pub struct Item {
    name: String,
    price: i32,
    quantity: i32,
}

impl Item {
    pub fn new(name: String, price: i32, quantity: i32) -> Self {
        Self {
            name,
            price,
            quantity,
        }
    }

    pub fn total(&self) -> i32 {
        self.price * self.quantity
    }
}

pub struct Invoice {
    order: Vec<Item>,
}

impl Invoice {
    pub fn new(order: Vec<Item>) -> Self {
        Self { order }
    }

    pub fn add(&mut self, order: Item) -> &mut Self {
        self.order.push(order);

        self
    }

    pub fn total(&self) -> i32 {
        self.order.iter().map(|item| item.total()).sum()
    }
}

impl Summary for Invoice {
    fn summarize(&self) -> anyhow::Result<()> {
        for item in &self.order {
            println!(
                "Name: {} with Quantity : {} Total: {}",
                item.name,
                item.quantity,
                item.total()
            )
        }

        Ok(())
    }
}

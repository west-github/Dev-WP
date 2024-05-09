#[derive(Debug)]
pub struct Product {
    name: String,
    price: u8,
}

impl Product {
    pub fn new(name: String, price: u8) -> Self {
        Self { name, price }
    }
}

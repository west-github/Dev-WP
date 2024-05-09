#[derive(Debug)]
pub struct InvoiceItem<'a> {
    pub name: &'a str,
    pub price: i32,
}

impl<'a> InvoiceItem<'a> {
    pub fn new(name: &'a str, price: i32) -> Self {
        Self { name, price }
    }
}

impl<'a> std::fmt::Display for InvoiceItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invoice Name: {} with Price: {}\n", self.name, self.price)
    }
}

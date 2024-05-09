#[derive(Debug)]
pub struct User {
    name: String,
    pub age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

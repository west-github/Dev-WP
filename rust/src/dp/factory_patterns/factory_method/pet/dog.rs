use super::Pet;
use crate::impl_pet;

#[derive(Debug)]
pub struct Dog<'a> {
    name: &'a str,
}

impl<'a> Dog<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl_pet!(Dog);

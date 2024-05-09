use super::Pet;
use crate::impl_pet;

#[derive(Debug)]
pub struct Cat<'a> {
    name: &'a str,
}

impl<'a> Cat<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl_pet!(Cat);

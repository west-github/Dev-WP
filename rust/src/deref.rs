use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Data<'a> {
    value: &'a str,
}

impl<'a> Data<'a> {
    fn new(value: &'a str) -> Self {
        Self { value }
    }
}

impl<'a> Deref for Data<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> DerefMut for Data<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(test)]
mod deref_tests {
    use super::Data;
    use anyhow::Result;

    #[test]
    fn deref_test() -> Result<()> {
        let mut data_pin = Box::pin(Data::new("Some value that we believe can't move"));

        data_pin.as_mut().get_mut().value = "Damn We change that";

        println!("{:?}", data_pin);

        let data_mut: &str = &mut data_pin.as_mut().get_mut();

        println!("{}", data_mut);

        let _box = Box::new(Data::new("West"));

        // pin::do_some_unsafe!(data_mut);

        Ok(())
    }
}

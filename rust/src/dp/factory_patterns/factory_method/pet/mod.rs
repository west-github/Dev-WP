use anyhow::Result;
mod cat;
mod dog;
mod pet_factory;

pub trait Pet {
    fn respond(&self) -> Result<()>;
}

#[macro_export]
macro_rules! impl_pet {
    ($name:ident) => {
        impl<'a> Pet for $name<'a> {
            fn respond(&self) -> anyhow::Result<()> {
                println!("{} is a good pet", self.name);

                Ok(())
            }
        }
    };
}

#[cfg(test)]
mod abstract_factory_tests {
    use super::{cat::Cat, dog::Dog, Pet};
    use anyhow::Result;

    // Static Dispatch
    fn take_pet_static(pet: impl Pet) -> Result<()> {
        pet.respond()?;

        Ok(())
    }

    // Static Dispatch
    fn take_pet_generic<T>(pet: T) -> Result<()>
    where
        T: Pet,
    {
        pet.respond()?;

        Ok(())
    }

    // Multiple Dispatch
    fn take_pet_multiple_dispatch(pet: &dyn Pet) -> Result<()> {
        pet.respond()?;

        Ok(())
    }

    #[test]
    fn pet_dispatch_tests() -> Result<()> {
        let dog = Dog::new("Yuggy");

        let cat = Cat::new("Baggy");

        // take_pet_static(dog)?;

        take_pet_multiple_dispatch(&dog)?;
        take_pet_multiple_dispatch(&cat)?;

        Ok(())
    }
}

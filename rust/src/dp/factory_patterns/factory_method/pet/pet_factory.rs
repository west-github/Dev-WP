use super::{cat::Cat, dog::Dog, Pet};

enum PetList<'a> {
    CAT(&'a str),
    DOG(&'a str),
}

struct PetFactory;

impl PetFactory {
    fn buy_pet(pet_type: PetList<'static>) -> Box<dyn Pet> {
        match pet_type {
            PetList::CAT(name) => Box::new(Cat::new(name)),
            PetList::DOG(name) => Box::new(Dog::new(name)),
        }
    }
}

#[test]
fn buy_pet() {
    let pet = PetFactory::buy_pet(PetList::CAT("Dongy"));

    // println!("{:?}", pet);
}

use super::{Burger, Malta, Restaurant};

pub struct Cheeka {
    name: String,
}

impl Cheeka {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Restaurant for Cheeka {
    type Burger = OrangeBurger;

    type Malta = OrangeMalta;

    fn create_burger(&self) -> Self::Burger {
        OrangeBurger {
            description: "Orange Pepper Mint".into(),
            name: "OPM".into(),
            price: 50,
        }
    }

    fn create_malta(&self) -> Self::Malta {
        OrangeMalta {
            description: "Orange Big".into(),
            name: "OB".into(),
            price: 50,
        }
    }
}
pub struct OrangeBurger {
    description: String,
    name: String,
    price: i32,
}

impl Burger for OrangeBurger {
    fn prepare(&self) {
        println!("{} ------- description", self.description);
        println!("{} is prepared", self.name);
        println!("Price: {}", self.price);
    }
}

pub struct OrangeMalta {
    description: String,
    name: String,
    price: i32,
}

impl Malta for OrangeMalta {
    fn prepare(&self) {
        println!("{} ------- description", self.description);
        println!("{} is prepared", self.name);
        println!("Price: {}", self.price);
    }
}

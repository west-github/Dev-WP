mod enum_example;

pub trait TraitStrategy {
    fn execute(&self);

    fn print(&self);
}

pub struct StructStrategy {
    operation: Box<dyn TraitStrategy>,
}

impl StructStrategy {
    pub fn new(operation: Box<dyn TraitStrategy>) -> Self {
        Self { operation }
    }

    pub fn execute(&self) {
        self.operation.execute()
    }

    pub fn print(&self) {
        self.operation.print()
    }

    pub fn set_strategy(&mut self, operation: Box<dyn TraitStrategy>) {
        self.operation = operation;
    }
}

#[derive(PartialEq, Eq)]
pub struct ONCE;

#[derive(PartialEq, Eq)]
pub struct BOTH;

#[derive(PartialEq, Eq)]
pub struct ALL;

macro_rules! impl_strategy {
    ($ident:ident) => {
        impl $ident {
            pub fn new() -> $ident {
                $ident {}
            }

            pub fn into_boxed() -> Box<$ident> {
                Box::new($ident::new())
            }
        }

        impl TraitStrategy for $ident {
            fn execute(&self) {
                println!("This is a {} operation", stringify!($ident).to_lowercase());
            }

            fn print(&self) {
                println!("Printing {} operation", stringify!($ident).to_lowercase());
            }
        }
    };
}

impl_strategy!(ONCE);
impl_strategy!(BOTH);
impl_strategy!(ALL);

#[cfg(test)]
mod struct_strategy__tests {
    use super::{StructStrategy, ALL, ONCE};

    #[test]
    fn test_one() {
        let mut strategy = StructStrategy::new(ONCE::into_boxed());

        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));

            strategy.print();
            strategy.execute();

            strategy.set_strategy(ALL::into_boxed());
        }
    }
}

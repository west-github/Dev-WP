use std::cell::RefCell;

use anyhow::Result;

use super::{unpaid::UnPaid, State};

pub struct Order {
    pub state: RefCell<Box<dyn State>>,
}

impl Order {
    pub fn new() -> Self {
        let state = RefCell::new(Box::new(UnPaid {}));

        Self { state }
    }

    fn recieve_payment(&self) -> Result<()> {
        self.state.borrow().recieve_payment()?;
        Ok(())
    }

    fn ship(&self) -> Result<()> {
        self.state.borrow().ship()?;
        Ok(())
    }

    fn mark_delivered(&self) -> Result<()> {
        self.state.borrow().mark_delivered(self)?;
        Ok(())
    }

    // Template Methods
    pub fn process(&mut self) -> Result<()> {
        self.recieve_payment()?;
        self.ship()?;
        self.mark_delivered()?;

        Ok(())
    }
}

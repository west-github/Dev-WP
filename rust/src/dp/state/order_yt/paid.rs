use super::{order::Order, State};
use anyhow::Result;

pub struct Paid;

impl State for Paid {
    fn recieve_payment(&self) -> Result<()> {
        println!("payment recieved");

        Ok(())
    }

    fn ship(&self) -> Result<()> {
        println!("Order enroute");

        Ok(())
    }

    fn mark_delivered(&self, order: &Order) -> Result<()> {
        println!("Order Shipped as payment was recieved");
        *order.state.borrow_mut() = Box::new(super::unpaid::UnPaid {});

        Ok(())
    }
}

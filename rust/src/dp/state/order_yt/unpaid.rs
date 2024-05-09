use super::{order::Order, paid::Paid, State};
use anyhow::Result;

pub struct UnPaid;

impl State for UnPaid {
    fn recieve_payment(&self) -> Result<()> {
        println!("payment yet to be recieved");

        Ok(())
    }

    fn mark_delivered(&self, order: &Order) -> Result<()> {
        println!("Order Shipped as payment was recieved");
        *order.state.borrow_mut() = Box::new(Paid {});

        Ok(())
    }

    fn ship(&self) -> Result<()> {
        println!("Order not delivered yet");

        Ok(())
    }
}

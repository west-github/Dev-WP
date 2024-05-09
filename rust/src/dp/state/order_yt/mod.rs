use anyhow::Result;

use self::order::Order;
mod order;
mod paid;
mod unpaid;

pub trait State {
    fn recieve_payment(&self) -> Result<()>;

    fn mark_delivered(&self, order: &Order) -> Result<()>;

    fn ship(&self) -> Result<()>;
}

#[cfg(test)]
mod state_tests {
    use super::order::Order;
    use anyhow::Result;

    #[test]
    fn test_one() -> Result<()> {
        let mut order = Order::new();

        loop {
            order.process()?;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

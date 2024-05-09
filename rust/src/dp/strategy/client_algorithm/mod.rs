use anyhow::Result;
mod client;
mod operation_one;
mod operation_two;

pub trait Algorithm {
    fn algorithm(&self) -> Result<()>;
}

#[cfg(test)]
mod strategy_test {
    use super::{client::Client, operation_one::OperationOne, operation_two::OperationTwo};
    use anyhow::Result;

    #[test]
    fn test_one() -> Result<()> {
        let operation_one = OperationOne::new("Yes we flying here in one");
        let mut client = Client::new(&operation_one);

        client.use_operation()?;

        let operation_two = OperationTwo::new("Yes we flying here in two");
        client.set_operation(&operation_two)?;

        client.use_operation()?;

        Ok(())
    }
}

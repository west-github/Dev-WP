use super::Algorithm;
use anyhow::Result;

pub struct OperationTwo<'a> {
    data: &'a str,
}

impl<'a> OperationTwo<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }
}

impl<'a> Algorithm for OperationTwo<'a> {
    fn algorithm(&self) -> Result<()> {
        println!("OperationTwo getting stuff done: {}", self.data);

        Ok(())
    }
}

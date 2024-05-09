use super::Algorithm;
use anyhow::Result;

pub struct OperationOne<'a> {
    data: &'a str,
}

impl<'a> OperationOne<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }
}

impl<'a> Algorithm for OperationOne<'a> {
    fn algorithm(&self) -> Result<()> {
        println!("OperationOne getting stuff done: {}", self.data);

        Ok(())
    }
}

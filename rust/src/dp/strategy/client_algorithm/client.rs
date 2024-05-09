use super::Algorithm;
use anyhow::Result;

pub struct Client<'a> {
    operation: &'a dyn Algorithm,
}

impl<'a> Client<'a> {
    pub fn new(operation: &'a dyn Algorithm) -> Self {
        Self { operation }
    }

    pub fn set_operation(&mut self, operation: &'a dyn Algorithm) -> Result<()> {
        self.operation = operation;

        Ok(())
    }

    pub fn use_operation(&self) -> Result<()> {
        self.operation.algorithm()?;

        Ok(())
    }
}

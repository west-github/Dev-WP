use self::psql::Psql;

mod psql;
mod redis;

pub trait DbManagement {
    // String can be anything even data
    fn get_string(self: Box<Self>) -> Box<dyn DbManagement>;
}

pub struct Application {
    state: Box<dyn DbManagement>,
}

impl Application {
    pub fn new() -> Self {
        let state = Box::new(Psql::new("psql".into()));

        Self { state }
    }

    pub fn get_string(mut self) {
        self.state = self.state.get_string();
    }
}

#[cfg(test)]
mod fsoc_tests {
    use anyhow::Result;

    #[test]
    fn test_one() -> Result<()> {
        Ok(())
    }
}

use super::TraitStrategy;

pub enum Strategy {
    ONCE,

    BOTH,

    ALL,
}

impl Strategy {
    pub fn set_strategy(&mut self, strategy: Strategy) {
        *self = strategy;
    }
}

impl TraitStrategy for Strategy {
    fn execute(&self) {
        match self {
            Strategy::ONCE => {
                println!("This is a once operation")
            }
            Strategy::BOTH => {
                println!("This is a both operation")
            }
            Strategy::ALL => {
                println!("This is a all operation")
            }
        }
    }

    fn print(&self) {
        match self {
            Strategy::ONCE => {
                println!("Printing once operation");
            }
            Strategy::BOTH => {
                println!("Printing both operation");
            }
            Strategy::ALL => {
                println!("Printing all operation");
            }
        }
    }
}

#[cfg(test)]
mod enum_strategy_tests {

    use super::Strategy;
    use crate::enum_over_dp::TraitStrategy;

    #[test]
    fn test_one() {
        let mut strategy = Strategy::ONCE;

        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));

            strategy.execute();
            strategy.print();

            if matches!(strategy, Strategy::ONCE) {
                strategy.set_strategy(Strategy::BOTH);
                continue;
            }

            if matches!(strategy, Strategy::BOTH) {
                strategy.set_strategy(Strategy::ALL);
                continue;
            }

            if matches!(strategy, Strategy::ALL) {
                strategy.set_strategy(Strategy::ONCE);
                continue;
            }
        }
    }
}

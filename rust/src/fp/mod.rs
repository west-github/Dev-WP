#[derive(Debug, PartialEq)]
pub struct Monads {
    value: i32,
}

impl Monads {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn map(&self, _fn: impl FnOnce(i32) -> i32) -> Self {
        Self::new(_fn(self.value))
    }
}

#[cfg(test)]
mod fp_tests {
    use super::Monads;

    #[test]
    fn fp_test() {
        let mut monads = Monads::new(20);

        monads = monads.map(|value| {
            assert_eq!(value, 20);

            value * 20
        });

        assert_eq!(monads, Monads::new(400));
    }
}

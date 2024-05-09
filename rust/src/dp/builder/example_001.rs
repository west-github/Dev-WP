#[derive(Debug)]
struct Data {
    value: String,
}

impl Data {
    fn new(value: String) -> Self {
        Self { value }
    }
}

struct DataBuilder {
    value: String,
}

impl DataBuilder {
    fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    fn with_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }

    fn build(&self) -> Data {
        let value = self.value.clone();

        Data::new(value)
    }
}

#[cfg(test)]
mod builder_tests {
    use super::DataBuilder;

    #[test]
    fn builder_test() -> anyhow::Result<()> {
        let data_builder = DataBuilder::new()
            .with_value(String::from("Yes we got it"))
            .build();

        println!("{:?}", data_builder);

        Ok(())
    }
}

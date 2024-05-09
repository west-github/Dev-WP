#[cfg(test)]
mod path_tests {
    use anyhow::Result;
    use std::path::Path;

    #[test]
    fn path_test() -> Result<()> {
        for component in Path::new("./").components() {
            println!("{component:?}");
        }

        let foo_bar = concat!("some words", "foo", "bar");

        println!("{foo_bar:?}");

        Ok(())
    }
}

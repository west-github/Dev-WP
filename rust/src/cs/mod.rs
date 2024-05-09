#[cfg(test)]
mod cs_tests {
    use anyhow::Result;

    #[test]
    fn cs_test() -> Result<()> {
        let lists = [20; 20];

        println!("{:?}", lists);
        Ok(())
    }

    #[test]
    fn match_moved() -> Result<()> {
        let value = String::new();

        match_fn(&Some(value))?;

        fn match_fn(value: &Option<String>) -> Result<()> {
            let _ = match value {
                Some(x) => "todo!()",
                None => "todo!()",
            };

            Ok(())
        }

        Ok(())
    }
}

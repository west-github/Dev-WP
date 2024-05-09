#[cfg(test)]
mod dir_entry_tests {
    use anyhow::Result;
    use std::{fs::DirBuilder, path::Path};

    #[test]
    fn dir_entry() -> Result<()>
    where
        String: AsRef<Path>,
    {
        assert!(!(<String as AsRef<Path>>::as_ref(&String::from("value")).is_dir()));

        let path = Path::new("value");

        let buf = path.to_path_buf();

        Ok(())
    }

    #[test]
    fn dir_builder() -> Result<()> {
        let _ = DirBuilder::new().recursive(true).create("some_dir")?;

        Ok(())
    }
}

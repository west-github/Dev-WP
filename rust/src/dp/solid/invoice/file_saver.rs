use crate::f;
use anyhow::Result;
use std::{fs::OpenOptions, io::Write};

pub trait FileSaverDataCollector {
    fn collector(&self) -> Result<String>;
}

pub struct FileSaver<'a> {
    path: &'a str,
    file_name: &'a str,
}

impl<'a> FileSaver<'a> {
    pub fn new(path: &'a str, file_name: &'a str) -> Self {
        Self { path, file_name }
    }

    pub fn save<T>(&self, data: T) -> Result<()>
    where
        T: FileSaverDataCollector,
    {
        let mut data = data.collector()?;

        data.push_str("\n\n");

        let mut file = OpenOptions::new().append(true).create(true).open(&f!(
            "{}/{}.txt",
            self.path,
            self.file_name
        ))?;

        file.write_all(data.as_bytes())?;

        Ok(())
    }
}

use std::fs::DirEntry;

use anyhow::anyhow;

pub struct Wrapper<T>(pub T);

impl<T> Wrapper<T> {
    pub fn new(t: T) -> Wrapper<T> {
        Wrapper(t)
    }
}

impl TryFrom<Wrapper<&DirEntry>> for String {
    type Error = anyhow::Error;

    fn try_from(value: Wrapper<&DirEntry>) -> Result<Self, Self::Error> {
        let res = value
            .0
            .path()
            .to_str()
            .map(String::from)
            .ok_or(anyhow!("Failed to convert entry to path"))?;

        Ok(res)
    }
}

impl From<Wrapper<&str>> for String {
    fn from(value: Wrapper<&str>) -> Self {
        value.0.to_owned()
    }
}

#[cfg(test)]
mod test_wrapper {
    use super::Wrapper;

    #[test]
    fn test_wrapper() -> anyhow::Result<()> {
        for entry in std::fs::read_dir("./")?.filter_map(|entry| entry.ok()) {
            let entry: String = Wrapper::new(&entry).try_into()?;

            println!("{:?}", entry);
        }

        Ok(())
    }

    #[test]
    fn test_string() {
        let str: String = Wrapper::new("hello").into();

        println!("{:?}", str);
    }
}

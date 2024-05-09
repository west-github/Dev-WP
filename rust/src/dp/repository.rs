#![allow(dead_code, unused_variables)]

use anyhow::Result;
use std::{thread, time::Duration};

trait DataRepository {
    fn get_data(&self) -> Result<Data<'_>>;
}

#[derive(Debug)]
struct Data<'a> {
    value: &'a str,
}

impl<'a> Data<'a> {
    fn new(value: &'a str) -> Self {
        Self { value }
    }
}
pub struct Psql;

impl DataRepository for Psql {
    fn get_data(&self) -> Result<Data> {
        thread::sleep(Duration::from_secs(2));

        Ok(Data::new("Some Value"))
    }
}

async fn use_data<D>(data: D) -> Result<()>
where
    D: DataRepository,
{
    let data = data.get_data()?;

    println!("{:?}", data);

    Ok(())
}

#[cfg(test)]
mod test_repository {
    use super::{use_data, Psql};
    use anyhow::Result;

    #[tokio::test]
    async fn test_one() -> Result<()> {
        let data = use_data(Psql).await?;

        Ok(())
    }
}

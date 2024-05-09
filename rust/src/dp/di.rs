use anyhow::Result;
use std::future::Future;

pub trait DataTraitInjection {
    fn data_execution(&self) -> impl Future<Output = Result<()>>;
}

pub struct DataInjection;

impl DataTraitInjection for DataInjection {
    async fn data_execution(&self) -> Result<()> {
        print!("This is a data DataInjection ");

        Ok(())
    }
}

pub struct FooInjection;

impl DataTraitInjection for FooInjection {
    async fn data_execution(&self) -> Result<()> {
        print!("This is a data FooInjection ");

        Ok(())
    }
}

#[derive(Debug)]
pub struct Data<S> {
    service: S,
    some_data: usize,
}

impl<S> Data<S>
where
    S: DataTraitInjection,
{
    pub fn new(service: S, some_data: usize) -> Self {
        Self { service, some_data }
    }

    pub async fn use_data(&self) -> Result<()> {
        let _ = self.service.data_execution().await?;

        println!("{}", self.some_data);

        Ok(())
    }
}

#[cfg(test)]
mod di_tests {
    use super::{Data, DataInjection, FooInjection};
    use anyhow::Result;

    #[tokio::test]
    async fn di_test() -> Result<()> {
        let data = Data::new(DataInjection {}, 10);

        data.use_data().await?;

        let data = Data::new(FooInjection {}, 20);

        data.use_data().await?;

        Ok(())
    }
}

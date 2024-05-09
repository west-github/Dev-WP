use anyhow::Result;

#[derive(Debug, Copy, Clone)]
pub struct Data<'a> {
    value: &'a str,
}

impl<'a> Data<'a> {
    pub fn new(value: &'a str) -> Self {
        Self { value }
    }
}

fn save_data<'a>(mut data: Data<'a>) -> Data<'a> {
    data.value = "OTher value";

    data
}

pub fn init() -> Result<()> {
    let data = Data::new("value");

    let res = save_data(data);

    println!("{:?} - {:?}", res, res.value);

    println!("{:?} - {:?}", data, data.value);

    Ok(())
}

#[cfg(test)]
mod copy_test {

    #[test]
    fn init() -> anyhow::Result<()> {
        assert_eq!((), super::init()?);

        Ok(())
    }
}

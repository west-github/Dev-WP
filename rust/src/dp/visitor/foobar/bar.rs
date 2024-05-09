use super::{FooBar, Visitor};
use anyhow::Result;

pub struct Bar {
    pub data: String,
}

pub fn bar(data: impl Into<String>) -> Bar {
    let data = data.into();

    Bar { data }
}

impl FooBar for Bar {
    fn accept(&self, object: &dyn Visitor) -> Result<()> {
        object.do_bar(self)?;

        Ok(())
    }
}

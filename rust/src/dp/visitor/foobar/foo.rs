use super::{FooBar, Visitor};
use anyhow::Result;

pub struct Foo {
    pub data: String,
}

pub fn foo(data: impl Into<String>) -> Foo {
    let data = data.into();

    Foo { data }
}

impl FooBar for Foo {
    fn accept(&self, object: &dyn Visitor) -> Result<()> {
        object.do_foo(self)?;

        Ok(())
    }
}

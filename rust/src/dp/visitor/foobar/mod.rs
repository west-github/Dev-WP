mod bar;
mod foo;

use anyhow::Result;
use bar::Bar;
use foo::Foo;

trait FooBar {
    fn accept(&self, object: &dyn Visitor) -> Result<()>;
}

trait Visitor {
    fn do_foo(&self, foo: &Foo) -> Result<()>;

    fn do_bar(&self, bar: &Bar) -> Result<()>;
}

pub struct Object {
    data: String,
}

impl Visitor for Object {
    fn do_foo(&self, foo: &Foo) -> Result<()> {
        println!("{:?} - {:?}", self.data, foo.data);

        Ok(())
    }

    fn do_bar(&self, bar: &Bar) -> Result<()> {
        println!("{:?} - {:?}", self.data, bar.data);

        Ok(())
    }
}

pub fn object(data: impl Into<String>) -> Object {
    let data = data.into();

    Object { data }
}

#[cfg(test)]
mod foobar_tests {
    use anyhow::Result;

    use super::{bar::bar, foo::foo, object, FooBar};

    #[test]
    fn foobar_test() -> Result<()> {
        let foobar = object("Container");

        foo("foo").accept(&foobar)?;
        bar("bar").accept(&foobar)?;

        Ok(())
    }
}

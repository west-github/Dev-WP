use anyhow::Result;

trait Some {
    fn is_some(&self) -> Result<()>;
}

trait SomeExt: Some + Sized {
    fn add_more(self) -> AddMore<Self>;
}

impl<S> SomeExt for S
where
    S: Some,
{
    fn add_more(self) -> AddMore<Self> {
        AddMore { svc: self }
    }
}

struct AddMore<S> {
    svc: S,
}
fn do_work() {
    for i in 0..10 {
        println!("{}", i);
    }
}

impl<S> Some for AddMore<S> {
    fn is_some(&self) -> Result<()> {
        do_work();

        Ok(())
    }
}

struct Foo;

impl Some for Foo {
    fn is_some(&self) -> Result<()> {
        do_work();

        Ok(())
    }
}

#[test]
fn decorator_test() -> anyhow::Result<()> {
    let foo = Foo {};
    foo.is_some()?;
    foo.add_more().is_some()?;

    Ok(())
}

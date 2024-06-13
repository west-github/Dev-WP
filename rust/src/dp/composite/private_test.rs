pub trait Summary {
    fn summarize(&mut self) -> anyhow::Result<()>;
}

pub trait Composite: Summary
where
    Self: Sized,
{
    // Considered as property
    fn store(&mut self) -> &mut Vec<Box<dyn Summary>>;

    fn add(&mut self, other: impl Summary + 'static) {
        self.store().push(Box::new(other));
    }
}

impl<T> Summary for T
where
    T: Composite,
{
    fn summarize(&mut self) -> anyhow::Result<()> {
        for b in self.store() {
            b.summarize()?;
        }

        Ok(())
    }
}

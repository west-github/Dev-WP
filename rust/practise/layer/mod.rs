mod define_layer_test;

pub trait Layer<S> {
    type Service;

    fn layer(&self, inner: S) -> Self::Service;
}

impl<'a, S, T> Layer<S> for &'a T
where
    T: ?Sized + Layer<S>,
{
    type Service = T::Service;

    fn layer(&self, inner: S) -> Self::Service {
        (**self).layer(inner)
    }
}

#[cfg(test)]
mod layer_tests {
    use anyhow::Result;

    #[test]
    fn test_layer() -> Result<()> {
        Ok(())
    }
}

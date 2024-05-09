use anyhow::Result;

struct Layer {
    data: String,
}

pub struct Router<T> {
    layer: Vec<Box<dyn Fn(&mut T) -> Result<()>>>,

    data: T,
}

impl<T> Router<T> {
    pub fn new(data: T) -> Self {
        Self {
            layer: vec![],
            data,
        }
    }

    pub fn layer(&mut self, func: Box<dyn Fn(&mut T) -> Result<()>>) -> &mut Self {
        self.layer.push(func);

        self
    }

    pub fn run(&mut self) -> Result<()> {
        if self.layer.len() == 0 {
            panic!("Can't run without layer")
        }

        for layer in &self.layer {
            layer(&mut self.data)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {

    #[allow(unused_doc_comments)]
    #[test]
    fn mw() -> anyhow::Result<()> {
        ///
        /// let router = Router::new()
        ///     .use((ctx) => {})
        ///     .use((ctx) => {});
        ///
        Ok(())
    }
}

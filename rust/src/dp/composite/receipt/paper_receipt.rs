use super::Summary;

pub struct PaperReceipt {
    children: Vec<Box<dyn Summary>>,
}

impl PaperReceipt {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn add(mut self, children: Box<dyn Summary>) -> Self {
        self.children.push(children);
        self
    }
}

impl Summary for PaperReceipt {
    fn summarize(&self) -> anyhow::Result<()> {
        for children in &self.children {
            self.summarize()?;
        }

        (!self.children.len() == 0).then(|| {
            println!("We got some values we need to work on");
        });

        Ok(())
    }
}

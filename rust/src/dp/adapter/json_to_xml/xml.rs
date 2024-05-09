use super::Save;
use anyhow::Result;

#[derive(Debug)]
pub struct Xml {
    data: String,
}

impl Xml {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl Save for Xml {
    fn save(&self) -> Result<()> {
        super::save(&self.data)
    }
}

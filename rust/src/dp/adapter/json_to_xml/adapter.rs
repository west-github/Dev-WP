use super::{json::Json, save, Save};

pub struct Adapter<'a>(pub &'a Json);

impl<'a> Save for Adapter<'a> {
    fn save(&self) -> anyhow::Result<()> {
        let string = String::from_utf8(self.0.data.into())?;

        save(&string)
    }
}

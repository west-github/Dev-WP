use super::Color;

pub struct Red {
    code: &'static str,
}

impl Red {
    pub fn new() -> Self {
        let code = "#e00000";

        Self { code }
    }
}

impl Color for Red {
    fn color(&self) {
        println!("We got this code color: {}", self.code);
    }
}

#[derive(Debug)]
pub struct Json {
    pub data: [u8; 4],
}

impl Json {
    pub fn new(data: [u8; 4]) -> Self {
        Self { data }
    }
}

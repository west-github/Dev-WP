use super::{psql::Psql, DbManagement};

pub struct Redis {
    conn: String,
}

impl Redis {
    pub fn new(conn: String) -> Self {
        Self { conn }
    }
}

impl DbManagement for Redis {
    fn get_string(self: Box<Self>) -> Box<dyn DbManagement> {
        Box::new(Psql::new("psql".into()))
    }
}

use super::{redis::Redis, DbManagement};

pub struct Psql {
    pub conn: String,
}

impl Psql {
    pub fn new(conn: String) -> Self {
        Self { conn }
    }
}

impl DbManagement for Psql {
    fn get_string(self: Box<Self>) -> Box<dyn DbManagement> {
        Box::new(Redis::new(String::from("redis")))
    }
}

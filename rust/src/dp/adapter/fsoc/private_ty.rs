use std::ops::Deref;

// ext
pub struct State<S>(pub S);

// ext
pub struct Psql(pub String);

impl Psql {
    pub fn new() -> Psql {
        Psql(String::from("This is a psql Connection"))
    }
}

// ext
pub struct Mongo(pub String);

impl Mongo {
    pub fn new() -> Mongo {
        Mongo(String::from("This is a mongo Connection"))
    }
}

impl<S> Deref for State<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

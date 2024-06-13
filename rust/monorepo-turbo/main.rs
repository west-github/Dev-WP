#![allow(non_camel_case_types, dead_code, unused_variables)]

struct user {
    name: String,
}

impl user {
    fn new(name: String) -> Self {
        Self { name }
    }
}

fn main() {
    let west = user::new(String::from("West"));
}

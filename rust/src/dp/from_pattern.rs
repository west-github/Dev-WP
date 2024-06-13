use anyhow::{anyhow, Result};
use std::fmt::Debug;

trait Bucket {
    type Output;

    fn get(&self, index: &'static str) -> Option<&Self::Output>;

    // Expected to return the key of where it's inserted
    fn push(&mut self, key: &'static str, value: Self::Output);
}

struct ListBucket {
    store: Vec<(&'static str, String)>,
}

impl ListBucket {
    fn new() -> Self {
        Self { store: vec![] }
    }
}

impl Bucket for ListBucket {
    type Output = String;

    fn get(&self, index: &'static str) -> Option<&Self::Output> {
        self.store
            .iter()
            .filter(|(i, _)| *i == index)
            .next()
            .map(|(_, value)| value)
    }

    fn push(&mut self, key: &'static str, value: Self::Output) {
        self.store.push((key, value));
    }
}

impl Debug for ListBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let logger = self
            .store
            .iter()
            .map(|(key, value)| format!("KEY: {} - DATA: {}", key, value))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", format!("{:>20}\n\n{}", "BUCKET DATA", logger))
    }
}

trait FromList
where
    Self: Sized,
{
    fn from_list(bucket: &mut impl Bucket<Output = String>) -> Result<Self>;
}

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}

impl User {
    fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
}

impl FromList for User {
    fn from_list(bucket: &mut impl Bucket<Output = String>) -> Result<Self> {
        let name = bucket.get("name").ok_or(anyhow!("Name is not inside Bucket"))?.clone();
        let age = bucket.get("age").ok_or(anyhow!("Age is not inside Bucket"))?.parse()?;

        Ok(User::new(name, age))
    }
}

fn get_from_list<T: FromList>(bucket: &mut ListBucket) -> Result<T> {
    T::from_list(bucket)
}

// test --package rust --lib -- dp::from_pattern::test --exact --show-output

#[test]
fn test() -> Result<()> {
    let mut bucket = ListBucket::new();
    bucket.push("name", String::from("West"));
    bucket.push("age", String::from("30"));
    // Even though it's duplicated we are returning the first
    bucket.push("name", String::from("East"));
    bucket.push("age", String::from("50"));

    println!("\n{:?}\n", bucket);

    let user: User = get_from_list(&mut bucket)?;
    println!("User with name {} - {}", user.name, user.age);

    Ok(())
}

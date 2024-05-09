use rust::string;
use std::fs;

pub trait ILogger {
    fn log(&self, content: String);
}

fn log(logger: impl ILogger, content: String) {
    logger.log(content);
}

pub struct User {
    name: String,
    email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }
}

/*
lorem
*/
pub struct FileLogger {
    path: String,
}
fn file_logger(path: String) -> FileLogger {
    FileLogger { path }
}

impl ILogger for FileLogger {
    fn log(&self, content: String) {
        let _ = fs::write(self.path.as_str(), content);
    }
}

fn main() {
    let user = User::new(string!("West"), string!("West@west2.com"));
    let fl = file_logger("./user.txt".into());

    log(fl, format!("Entry - User: name: {} email: {}", user.name, user.email));

    fn recur(lists: &[i32]) {
        match lists {
            [one] => println!("We got value: {}", one),

            &[ref head @ .., tail] if !head.is_empty() => {
                println!("We got value: {}", tail);

                recur(head)
            }
            _ => (),
        }
    }

    recur(&vec![1, 2, 3, 4, 5]);

    let data = Data::new("West".into());

    take_str(&data);

    fn take_str(value: &str) {
        println!("We got a value of: {}", value);
    }

    fn use_result() -> Result<Data, &'static str> {
        Ok(Data::new(string!("west in the building")))
    }

    let value = 10;

    fn on_auth(Data { content }: Data, value: i32) -> () {
        println!("We got content: {} value: {}", content, value);
    }

    return use_result()
        .map(|data| on_auth(data, value))
        .unwrap_or_else(|err| println!("We got this error: {}", err));
}

#[allow(dead_code)]
mod new_pattern {
    trait Foo {
        fn use_foo(&self);
    }

    fn use_foo<T: Foo>(foo: T) {
        foo.use_foo();
    }

    struct UseFoo;

    impl Foo for UseFoo {
        fn use_foo(&self) {
            println!("We are using foo aren't we")
        }
    }

    #[test]
    fn test_use_foo() {
        let _uf = UseFoo {};

        use_foo(_uf);
    }
}

pub struct Data {
    content: String,
}

impl Data {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl std::ops::Deref for Data {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

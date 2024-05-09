mod foobar;

#[derive(Debug)]
struct Data {
    name: String,
}

impl Data {
    fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
struct Other {
    content: String,
}

impl Other {
    fn new(content: String) -> Self {
        Self { content }
    }
}

trait Visitor {
    fn accept(&self, func: impl Fn(String) -> String) -> Result<(), &'static str>;
}

impl Visitor for Data {
    fn accept(&self, func: impl Fn(String) -> String) -> Result<(), &'static str> {
        let value = func(self.name.to_owned());

        println!("{}", value);

        Ok(())
    }
}

impl Visitor for Other {
    fn accept(&self, func: impl Fn(String) -> String) -> Result<(), &'static str> {
        let value = func(self.content.to_owned());

        println!("{}", value);

        Ok(())
    }
}

#[test]
fn test_visitor() {
    let data = Data::new("Some Data".to_owned());

    let _ = data.accept(|value| {
        println!("{} in a visitor closure", value);

        value
    });
}

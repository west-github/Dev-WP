#[allow(dead_code)]
struct Data {
    #[cfg(feature = "dev")]
    name: String,
}

impl Data {
    #[cfg(feature = "dev")]
    fn new(name: String) -> Self {
        Self { name }
    }
}

fn main() {
    #[cfg(feature = "dev")]
    {
        use rust::string;
        let data = Data::new(string!("West"));

        println!("{}", data.name);
    }
}

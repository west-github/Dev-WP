#![allow(dead_code)]

pub trait ILogger {
    fn log(&self, content: String);
}

pub fn log(logger: impl ILogger, content: String) {
    logger.log(content);
}

#[derive(Copy, Clone)]
pub struct Console;

impl ILogger for Console {
    fn log(&self, content: String) {
        println!("{}", content)
    }
}

fn info(content: String) -> String {
    format!("***GREEN: {}***", content)
}

fn error(content: String) -> String {
    format!("***RED: {}***", content)
}

// Hacker X Terminal Hacker Theme, Lua dark theme

// Rebecca Dark

// Slab 2077 Pro Amoled Black Shiny

#[test]
fn solid_test() {
    use rust::string;

    let (info, error) = console_logger();

    info(string!("Soldiers aree coming"));
    error(string!("British are coming"));
}

fn console_logger() -> (Box<dyn Fn(String)>, Box<dyn Fn(String)>) {
    let cl = Console;

    // Adapter Used Here
    (
        Box::new(move |content: String| log(cl, info(content))),
        Box::new(move |content: String| log(cl, error(content))),
    )
}

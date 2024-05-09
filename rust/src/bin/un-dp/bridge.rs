pub trait ILoggger {
    fn log(&self, msg: &str);
}

pub struct Console;

impl ILoggger for Console {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

pub struct Logger {
    logger: Box<dyn ILoggger>,
}

impl Logger {
    pub fn new(logger: Box<dyn ILoggger>) -> Self {
        Self { logger }
    }
}

pub struct Client {
    logger: Logger,
}

impl Client {
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }

    pub fn log(&self, msg: &str) {
        self.logger.logger.log(msg);
    }
}

#[test]
fn test() {
    let logger = Client::new(Logger::new(Box::new(Console)));

    logger.log("Yes we got it");
}

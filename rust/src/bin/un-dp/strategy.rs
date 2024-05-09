pub trait ILoggger {
    fn log(&self, msg: &str);
}

pub struct Console;

impl ILoggger for Console {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

pub struct Client {
    logger: Box<dyn ILoggger>,
}

impl Client {
    pub fn new(logger: Box<dyn ILoggger>) -> Self {
        Self { logger }
    }

    fn log(&self, msg: &str) {
        self.logger.log(msg)
    }
}

#[test]
fn test() {
    let logger = Client::new(Box::new(Console));

    logger.log("Yes we got it");
}

// pub fn log(logger: impl ILoggger, msg: &str) {
//     logger.log(msg);
// }

// log(Console, "Yes we got it");

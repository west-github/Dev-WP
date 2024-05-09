#![allow(dead_code, unused_variables)]
pub struct Reciever {
    action: String,
}

impl Reciever {
    pub fn new(action: String) -> Self {
        Self { action }
    }
}

pub trait ICommand {
    fn undo(&self);

    fn execute(&self);
}

pub struct Copy;

impl ICommand for Copy {
    fn undo(&self) {
        println!("Undoing Action....")
    }

    fn execute(&self) {
        println!("Executing Action....")
    }
}

pub struct Client {
    action: Vec<Box<dyn ICommand>>,
}

impl Client {
    pub fn new() -> Self {
        Self { action: vec![] }
    }

    pub fn add(&mut self, action: Box<dyn ICommand>) {
        self.action.push(action);
    }

    pub fn execute(&self) {
        for action in &self.action {
            action.execute();
        }
    }

    pub fn rollback(&self) {
        for action in self.action.iter() {
            action.undo();
        }
    }
}

#[test]
fn test_command() {
    let mut cmd_client = Client::new();

    cmd_client.add(Box::new(Copy));
    cmd_client.add(Box::new(Copy));
    cmd_client.add(Box::new(Copy));

    cmd_client.execute();

    cmd_client.rollback();
}

use std::cell::RefCell;

pub trait Mediator {
    fn notify(&self, ev: &str);
}

pub struct One<'a> {
    mediator: Option<&'a dyn Mediator>,
}

impl<'a> One<'a> {
    pub fn new(mediator: Option<&'a dyn Mediator>) -> Self {
        Self { mediator }
    }

    pub fn reaction(&self) {
        println!("One Called");
        self.mediator.map(|m| m.notify("ONE"));
    }

    pub fn set_mediator(&mut self, mediator: &'a dyn Mediator) {
        self.mediator = Some(mediator);
    }
}

pub struct Two<'a> {
    mediator: Option<&'a dyn Mediator>,
}

impl<'a> Two<'a> {
    pub fn new(mediator: Option<&'a dyn Mediator>) -> Self {
        Self { mediator }
    }

    pub fn reaction(&self) {
        println!("One Called");
        self.mediator.map(|m: &dyn Mediator| m.notify("TWO"));
    }

    pub fn set_mediator(&mut self, mediator: &'a dyn Mediator) {
        self.mediator = Some(mediator);
    }
}

pub struct Client<'a> {
    one: &'a RefCell<One<'a>>,
    two: &'a RefCell<Two<'a>>,
}

impl<'a> Mediator for Client<'a> {
    fn notify(&self, ev: &str) {
        println!("{}", ev);
    }
}

#[test]
fn name() {
    let one = RefCell::new(One::new(None));
    let two = RefCell::new(Two::new(None));

    let client = Client { one: &one, two: &two };

    one.borrow_mut().set_mediator(&client);
    two.borrow_mut().set_mediator(&client);

    one.borrow().reaction();
    two.borrow().reaction();
}

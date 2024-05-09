// use anyhow::Result;

// pub trait Handler {
//     fn execute(&self) -> Result<()>;

//     fn next(self, next: Box<dyn Handler>) -> Box<dyn Handler>;
// }

// #[derive(Default)]
// pub struct Router {
//     handler: Option<Box<dyn Handler>>,
// }

// impl Handler for Router {
//     fn execute(&self) -> Result<()> {
//         todo!()
//     }

//     fn next(self, next: Box<dyn Handler>) -> Box<dyn Handler> {
//         todo!()
//     }
// }

// pub struct Order {
//     name: String,
// }

// impl Order {
//     pub fn new(name: String) -> Self {
//         Self { name }
//     }
// }

// impl Handler for Order {
//     fn execute(&self) -> Result<()> {
//         println!("Executing the order {}", self.name);

//         Ok(())
//     }

//     fn next(self, next: Box<dyn Handler>) -> Box<dyn Handler> {
//         todo!()
//     }
// }

// #[cfg(test)]
// mod crp_tests {
//     use anyhow::Result;

//     use super::{Handler, Order, Router};

//     #[test]
//     fn crp_test() -> Result<()> {
//         let router = Router::default()
//             .next(Box::new(Order::new(String::from("Order 1"))))
//             .next(Box::new(Order::new(String::from("Order 2"))));

use anyhow::{anyhow, Result};

trait RouterNode {
    fn process(&self) -> Result<()>;
}

struct Order {
    order_name: String,
}

impl Order {
    fn new(order_name: String) -> Self {
        Order { order_name }
    }
}

impl RouterNode for Order {
    fn process(&self) -> Result<()> {
        println!("Processing order: {}", self.order_name);

        Ok(())
    }
}

struct Order2 {
    order_name: String,
}

impl Order2 {
    fn new(order_name: String) -> Self {
        Order2 { order_name }
    }
}

impl RouterNode for Order2 {
    fn process(&self) -> Result<()> {
        println!("Processing order: {}", self.order_name);

        Err(anyhow!("Yes we got an error"))
    }
}

struct Router {
    nodes: Vec<Box<dyn RouterNode>>,
}

impl Router {
    fn default() -> Self {
        Router { nodes: Vec::new() }
    }

    fn next(mut self, node: Box<dyn RouterNode>) -> Self {
        self.nodes.push(node);
        self
    }

    fn process(&self) -> Result<()> {
        println!("Router Processing");

        for node in &self.nodes {
            node.process()?;
        }

        Ok(())
    }
}

#[test]
fn crp_test() -> Result<()> {
    let router = Router::default()
        .next(Box::new(Order2::new(String::from("Order 1"))))
        .next(Box::new(Order::new(String::from("Order 2"))));

    router.process()?;

    Ok(())
}

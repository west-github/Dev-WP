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

    fn next(mut self, node: impl RouterNode + 'static) -> Self {
        self.nodes.push(Box::new(node));
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
        .next(Order::new(String::from("Order 1")))
        .next(Order::new(String::from("Order 2")));

    if let Err(err) = router.process() {
        println!("Error Occured: {}", err);
    }

    Ok(())
}

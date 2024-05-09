mod cheeka;

trait Burger {
    fn prepare(&self);
}

trait Malta {
    fn prepare(&self);
}

enum Order {
    Burger,
    Malta,
}

trait Restaurant {
    type Burger: Burger;

    type Malta: Malta;

    fn create_burger(&self) -> Self::Burger;

    fn create_malta(&self) -> Self::Malta;

    fn order_burger(&self) {
        let burger = self.create_burger();
        burger.prepare();
    }

    fn order_malta(&self) {
        let malta = self.create_malta();
        malta.prepare();
    }
}

fn request(order: Order, _res: impl Restaurant) {
    match order {
        Order::Burger => _res.order_burger(),
        Order::Malta => _res.order_malta(),
    }
}

#[cfg(test)]
mod test {
    use super::{cheeka::Cheeka, request, Order};

    #[test]
    fn test_cheeka() {
        request(Order::Burger, Cheeka::new("cheeka".into()))
    }
}

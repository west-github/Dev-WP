pub trait Database {
    fn new() -> Self;
}

#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn is_valid(&self) -> bool {
        self.name.chars().any(|c| c == 'e')
    }
}

impl Database for User {
    fn new() -> Self {
        User::new(String::from("West"))
    }
}

#[derive(Debug, PartialEq)]
pub struct Product {
    pub name: String,
}

impl Product {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Database for Product {
    fn new() -> Self {
        Product::new(String::from("Tomato"))
    }
}

#[derive(Debug)]
pub struct DB<T> {
    pub(crate) model: T,
}

impl<T> DB<T> {
    pub fn new(model: T) -> Self {
        Self { model }
    }

    pub fn get<U>(&self) -> U
    where
        U: Database,
    {
        U::new()
    }
}

#[derive(Debug)]
pub struct Context {
    pub product: DB<Product>,
    pub user: DB<User>,
}

impl Context {
    pub fn new() -> Self {
        let user = DB::new(User::new("East".into()));
        let product = DB::new(Product::new("Orange".into()));

        Self { user, product }
    }
}

#[cfg(test)]
mod ctx_tests {
    use super::{Context, Product, User};

    #[test]
    fn ctx_test() {
        let ctx = Context::new();

        println!("{:#?}", ctx);

        let user = ctx.user.get::<User>();

        assert_eq!(user.is_valid(), true);

        assert_eq!(user, User::new("West".into()));

        let product = ctx.product.get::<Product>();

        assert_eq!(product, Product::new("Tomato".into()));
    }
}

#![allow(dead_code, unused_variables)]
use anyhow::{anyhow, Result};
use derive_new::new;

#[derive(new)]
pub struct User {
    pub name: String,
}

#[derive(new)]
pub struct Context {
    mode: i32,
    user: Option<User>,
}

impl Context {
    #[allow(dead_code)]
    pub const VERSION: &'static str = "0.1";

    pub fn user(&self) -> Result<Option<&User>> {
        if self.mode == 0 {
            return Err(anyhow!("We are in maintenance Mode"));
        }

        Ok(self.user.as_ref())
    }
}

pub struct Product {
    name: String,
    amount: i32,
}

impl Product {
    pub fn new(pd: (String, i32)) -> Self {
        let name = pd.0;
        let amount = pd.1;

        Self { name, amount }
    }
}

impl Management for Product {
    fn do_management(&self) -> Result<()> {
        println!("NAME: {} - AMOUNT: {}", self.name, self.amount);

        Ok(())
    }
}

impl Collect for Product {
    fn make_collect(&self) -> String {
        format!("Name: {} - Age: {}", self.name, self.amount)
    }
}

pub trait Management {
    fn do_management(&self) -> Result<()>;
}

pub trait Collect {
    fn make_collect(&self) -> String;
}

#[macro_export]
macro_rules! s {
    ($value:expr) => {
        String::from($value)
    };
}

pub trait Payment {
    fn get_payment(&self) -> i32;
}

#[derive(new)]
pub struct Solder<T: Payment> {
    status: T, // - snip
}

impl<T: Payment> Solder<T> {
    pub fn get_payment(&self) {
        let amount = self.status.get_payment();

        println!("{}", amount);
    }
}

pub struct Dead {}
pub struct Alive {}

macro_rules! impl_payment {
    ($ident:ident) => {
        impl Payment for $ident {
            fn get_payment(&self) -> i32 {
                30
            }
        }
    };
}

impl_payment!(Alive);
impl_payment!(Dead);

fn do_things<T>(ctx: &Context, mng: T) -> Result<()>
where
    T: IntoIterator,
    T::Item: Management,
{
    let user = ctx.user()?.ok_or(anyhow!("Hit the snag"))?;
    println!("This user with name: {} is performinig an action", user.name);

    for m in mng {
        m.do_management()?;
    }

    Ok(())
}

fn do_useful<T: Collect>(value: &T) -> Result<()> {
    let value = value.make_collect();

    // Do Something useful but we can literall pass anything that implement collect
    println!("{}", value);
    Ok(())
}

#[test]
pub fn test_helper() -> Result<()> {
    let ctx = Context::new(1, Some(User::new(s!("West"))));
    let prod = [
        (s!("Orange"), 30),
        (s!("Orange"), 50),
        (s!("Grape"), 20),
        (s!("Water"), 10),
        (s!("Macaroni"), 80),
    ];

    do_useful(&prod.clone().map(Product::new)[0])?;
    do_things(&ctx, prod.map(Product::new))?;

    println!("{}", Context::VERSION);

    let west = Solder::new(Alive {});
    west.get_payment();

    Ok(())
}

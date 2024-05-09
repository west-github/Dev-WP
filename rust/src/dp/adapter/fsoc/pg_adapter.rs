use super::{private_ty::Psql, product::Product, user::User, ProductManagement, UserManagement};

pub struct PgAdapter(pub Psql);

impl PgAdapter {
    pub fn new(conn: Psql) -> PgAdapter {
        PgAdapter(conn)
    }
}

impl UserManagement for PgAdapter {
    // Fetch User from database
    async fn get_user(&self) -> User {
        println!("{}", self.0 .0);

        User::new(String::from("East"), 20)
    }
}

impl ProductManagement for PgAdapter {
    // Fetch product from database
    async fn get_product(&self) -> Product {
        Product::new("Apple".into(), 40)
    }
}

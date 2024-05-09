use super::{private_ty::Mongo, product::Product, user::User, ProductManagement, UserManagement};

pub struct MongoAdapter(pub Mongo);

impl MongoAdapter {
    pub fn new(conn: Mongo) -> MongoAdapter {
        MongoAdapter(conn)
    }
}

impl UserManagement for MongoAdapter {
    // Fetch User from database
    async fn get_user(&self) -> User {
        println!("{}", self.0 .0);

        User::new(String::from("West"), 30)
    }
}

impl ProductManagement for MongoAdapter {
    // Fetch product from database
    async fn get_product(&self) -> Product {
        Product::new("Tomato".into(), 50)
    }
}

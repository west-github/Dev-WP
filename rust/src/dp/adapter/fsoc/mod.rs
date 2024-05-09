use self::{product::Product, user::User};
use std::future::Future;
mod mongo_adapter;
mod pg_adapter;
mod private_ty;
mod product;
mod user;

pub trait UserManagement {
    fn get_user(&self) -> impl Future<Output = User>;
}

pub trait ProductManagement {
    fn get_product(&self) -> impl Future<Output = Product>;
}

#[cfg(test)]
mod adapter_tests {

    use super::{
        mongo_adapter::MongoAdapter,
        pg_adapter::PgAdapter,
        private_ty::{Mongo, Psql, State},
        ProductManagement, UserManagement,
    };

    async fn call_get_user<S>(db: &State<S>)
    where
        S: UserManagement,
    {
        let user = db.get_user().await;

        println!("{:?}", user);
    }

    async fn call_get_product<S>(db: &State<S>)
    where
        S: ProductManagement,
    {
        let product = db.get_product().await;

        println!("{:?}", product);
    }

    #[tokio::test]
    async fn adapter_test() -> anyhow::Result<()> {
        let db = State(PgAdapter::new(Psql::new()));

        call_get_user(&db).await;
        call_get_product(&db).await;

        let db = State(MongoAdapter::new(Mongo::new()));

        println!("\n");

        call_get_user(&db).await;
        call_get_product(&db).await;

        Ok(())
    }
}

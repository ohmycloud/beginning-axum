pub use sea_orm_migration::prelude::*;

mod m20260808_054714_create_users;
mod m20260808_062919_create_category;
mod m20260808_063526_create_product;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260808_054714_create_users::Migration),
            Box::new(m20260808_062919_create_category::Migration),
            Box::new(m20260808_063526_create_product::Migration),
        ]
    }
}

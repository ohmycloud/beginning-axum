use crate::m20260808_062919_create_category::Category;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Product {
    Table,
    Id,
    Title,
    Price,
    Category,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260808_063526_create_product"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Title).string().not_null())
                    .col(ColumnDef::new(Product::Price).integer().not_null())
                    .col(ColumnDef::new(Product::Category).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_category")
                            .from(Product::Table, Product::Category)
                            .to(Category::Table, Category::Name),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).if_exists().to_owned())
            .await?;
        Ok(())
    }
}

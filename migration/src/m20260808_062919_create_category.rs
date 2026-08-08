use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Category {
    Table,
    Name,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260808_062919_create_category"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Name)
                            .string()
                            .unique_key()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).if_exists().to_owned())
            .await?;
        Ok(())
    }
}

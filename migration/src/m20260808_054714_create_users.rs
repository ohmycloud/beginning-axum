use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Username,
    Password,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260808_054714_create_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table) // `Users`
                    .if_not_exists() // IF NOT EXISTS
                    .col(
                        ColumnDef::new(Users::Id) // `Id`
                            .integer() // INT
                            .not_null() // NOT NULL
                            .auto_increment() // AUTO_INCREMENT
                            .primary_key(), // PRIMARY KEY
                    )
                    // `Username` VARCHAR(255) NOT NULL
                    .col(ColumnDef::new(Users::Username).string().not_null())
                    // `Password` VARCHAR(255) NOT NULL
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    // Returns `Table` object that created `Users` table and defined columns
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).if_exists().to_owned())
            .await?;
        Ok(())
    }
}

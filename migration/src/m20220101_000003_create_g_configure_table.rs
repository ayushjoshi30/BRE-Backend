use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000003_create_g_configure_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(g_configure::Table)
                    .if_not_exists()
                    .col(pk_auto(g_configure::Id))
                    .col(string(g_configure::UserName))
                    .col(string(g_configure::Password))
                    .col(string(g_configure::BucketName))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(g_configure::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum g_configure {
    Table,
    Id,
    UserName,
    Password,
    BucketName,
}

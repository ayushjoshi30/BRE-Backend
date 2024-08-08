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
                    .table(GConfigure::Table)
                    .if_not_exists()
                    .col(pk_auto(GConfigure::Id))
                    .col(string(GConfigure::username))
                    .col(string(GConfigure::Password))
                    .col(string(GConfigure::BucketName))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GConfigure::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GConfigure {
    Table,
    Id,
    username,
    Password,
    BucketName,
}

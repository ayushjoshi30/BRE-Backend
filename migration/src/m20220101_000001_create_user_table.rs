use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string(Users::Username).unique_key())
                    .col(string(Users::WorkspaceId))
                    .col(string(Users::Password))
                    .col(string(Users::Role))
                    .col(date_time(Users::CreatedAt).default("now()"))
                    .col(date_time(Users::UpdatedAt).default("now()"))
                    .col(date_time(Users::LastLogin))
                    .col(boolean(Users::IsDeleted).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Password,
    Role,
    WorkspaceId,
    CreatedAt,
    UpdatedAt,
    LastLogin,
    IsDeleted,
}

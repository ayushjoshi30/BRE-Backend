use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_g_appusers_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(g_appusers::Table)
                    .if_not_exists()
                    .col(pk_auto(g_appusers::Id))
                    .col(string(g_appusers::FirstName).not_null())
                    .col(string(g_appusers::LastName).not_null())
                    .col(string(g_appusers::UserName).not_null().unique_key())
                    .col(string(g_appusers::Email).not_null().unique_key())
                    .col(string(g_appusers::MobileNo).not_null().unique_key())
                    .col(
                        date_time(g_appusers::CreatedOnDate)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(integer(g_appusers::WorkspaceId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workspace_id")  // Ensure a unique name for the foreign key
                            .from(g_appusers::Table, g_appusers::WorkspaceId)
                            .to(g_workspaces::Table, g_workspaces::Id),
                    )
                    .col(boolean(g_appusers::IsActive).default(true))
                    .col(boolean(g_appusers::IsDeleted).default(false))
                    .col(
                        date_time(g_appusers::LastLogin)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(string(g_appusers::Password).not_null())
                    .col(boolean(g_appusers::IsAdmin).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(g_appusers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum g_appusers {
    Table,
    Id,
    FirstName,
    LastName,
    UserName,
    Email,
    MobileNo,
    CreatedOnDate,
    WorkspaceId,
    IsActive,
    IsDeleted,
    LastLogin,
    Password,
    IsAdmin,
}

#[derive(DeriveIden)]
enum g_workspaces {
    Table,
    Id,
}

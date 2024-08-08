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
                    .table(GAppusers::Table)
                    .if_not_exists()
                    .col(pk_auto(GAppusers::Id))
                    .col(string(GAppusers::FirstName).not_null())
                    .col(string(GAppusers::LastName).not_null())
                    .col(string(GAppusers::UserName).not_null().unique_key())
                    .col(string(GAppusers::Email).not_null().unique_key())
                    .col(string(GAppusers::MobileNo).not_null().unique_key())
                    .col(
                        date_time(GAppusers::CreatedOnDate)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(integer(GAppusers::WorkspaceId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workspace_id")  // Ensure a unique name for the foreign key
                            .from(GAppusers::Table, GAppusers::WorkspaceId)
                            .to(GWorkspaces::Table, GWorkspaces::Id),
                    )
                    .col(boolean(GAppusers::IsActive).default(true))
                    .col(boolean(GAppusers::IsDeleted).default(false))
                    .col(
                        date_time(GAppusers::LastLogin)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(string(GAppusers::Password).not_null())
                    .col(boolean(GAppusers::IsAdmin).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GAppusers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GAppusers {
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
enum GWorkspaces {
    Table,
    Id,
}

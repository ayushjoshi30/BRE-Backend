use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000002_create_g_releases_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(g_releases::Table)
                    .if_not_exists()
                    .col(pk_auto(g_releases::Id))
                    .col(string(g_releases::Version).not_null())
                    .col(string(g_releases::FilePath).not_null())
                    .col(json_binary(g_releases::FileJson).not_null())
                    .col(date_time(g_releases::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(boolean(g_releases::IsReleased).default(false))
                    .col(date_time(g_releases::ReleasedDate))
                    .col(integer(g_releases::CreatedByUser).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("CreatedByUser")
                            .from(g_releases::Table, g_releases::CreatedByUser)
                            .to(g_appusers::Table, g_appusers::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(g_releases::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum g_releases {
    Table,
    Id,
    Version,
    FilePath,
    FileJson,
    CreatedAt,
    IsReleased,
    ReleasedDate,
    CreatedByUser,
}

#[derive(DeriveIden)]
enum g_appusers {
    Table,
    Id,
}


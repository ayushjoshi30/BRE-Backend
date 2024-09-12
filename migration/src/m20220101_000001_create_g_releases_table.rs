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
                    .table(GReleases::Table)
                    .if_not_exists()
                    .col(pk_auto(GReleases::Id))
                    .col(string(GReleases::Version).not_null())
                    .col(integer(GReleases::WorkspaceId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workspace_id")
                            .from(GReleases::Table, GReleases::WorkspaceId)
                            .to(GWorkspaces::Table, GWorkspaces::Id),
                    )
                    .col(string(GReleases::FilePath).not_null())
                    .col(json_binary(GReleases::FileJson).not_null())
                    .col(date_time(GReleases::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(boolean(GReleases::IsReleased).default(false))
                    .col(date_time(GReleases::ReleasedDate))
                    .col(integer(GReleases::CreatedByUser).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("CreatedByUser")
                            .from(GReleases::Table, GReleases::CreatedByUser)
                            .to(GAppusers::Table, GAppusers::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GReleases::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GReleases {
    Table,
    Id,
    Version,
    WorkspaceId,
    FilePath,
    FileJson,
    CreatedAt,
    IsReleased,
    ReleasedDate,
    CreatedByUser,
}

#[derive(DeriveIden)]
enum GAppusers {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum GWorkspaces {
    Table,
    Id,
}
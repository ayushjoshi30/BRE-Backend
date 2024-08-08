use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_g_rules_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(g_rules::Table)
                    .if_not_exists()
                    .col(pk_auto(g_rules::Id))
                    .col(integer(g_rules::WorkspaceId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workspace_id")
                            .from(g_rules::Table, g_rules::WorkspaceId)
                            .to(g_workspaces::Table, g_workspaces::Id),
                    )
                    .col(string(g_rules::RulePath).not_null())
                    .col(json_binary(g_rules::RuleJson).not_null())
                    .col(integer(g_rules::CreatedByUser).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_createdbyuser_id")
                            .from(g_rules::Table, g_rules::CreatedByUser)
                            .to(g_appusers::Table, g_appusers::Id),
                    )
                    .col(timestamp(g_rules::LastUpdated).not_null().default("now()"))
                    .col(string(g_rules::DraftFilePath))
                    .col(json_binary(g_rules::DraftFileJson))
                    .col(boolean(g_rules::IsDraft).default(false))
                    .col(timestamp(g_rules::PublishedAt))
                    .col(string(g_rules::Version))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(g_rules::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum g_rules {
    Table,
    Id,
    WorkspaceId,
    RulePath,
    RuleJson,
    CreatedByUser,
    LastUpdated,
    DraftFilePath,
    DraftFileJson,
    IsDraft,
    PublishedAt,
    Version,
}


#[derive(DeriveIden)]
enum g_workspaces {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum g_appusers {
    Table,
    Id,
}
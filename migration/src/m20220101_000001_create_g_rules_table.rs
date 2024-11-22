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
                    .table(GRules::Table)
                    .if_not_exists()
                    .col(pk_auto(GRules::Id))
                    .col(integer(GRules::WorkspaceId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workspace_id")
                            .from(GRules::Table, GRules::WorkspaceId)
                            .to(GWorkspaces::Table, GWorkspaces::Id),
                    )
                    .col(string(GRules::RulePath).not_null())
                    .col(json_binary(GRules::RuleJson).not_null())
                    .col(integer(GRules::CreatedByUser).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_createdbyuser_id")
                            .from(GRules::Table, GRules::CreatedByUser)
                            .to(GAppusers::Table, GAppusers::Id),
                    )
                    .col(timestamp(GRules::LastUpdated).not_null().default("now()"))
                    .col(string(GRules::DraftFilePath))
                    .col(json_binary(GRules::DraftFileJson))
                    .col(boolean(GRules::IsDraft).default(false))
                    .col(timestamp(GRules::PublishedAt))
                    .col(string(GRules::Version))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GRules::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GRules {
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
enum GWorkspaces {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum GAppusers {
    Table,
    Id,
}
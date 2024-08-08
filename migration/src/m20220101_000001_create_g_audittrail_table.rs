use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000004_create_g_audittrail_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(g_audittrail::Table)
                    .if_not_exists()
                    .col(pk_auto(g_audittrail::Id))
                    .col(string(g_audittrail::Action).not_null())
                    .col(integer(g_audittrail::WorkspaceId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("WorkspaceId")
                            .from(g_audittrail::Table, g_audittrail::WorkspaceId)
                            .to(g_workspaces::Table, g_workspaces::Id),                          
                    )
                    .col(timestamp_with_time_zone(g_audittrail::Timestamp).not_null().default(Expr::current_timestamp()))
                    .col(text(g_audittrail::Details).not_null())
                    .col(integer(g_audittrail::RuleId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rule_id")
                            .from(g_audittrail::Table, g_audittrail::RuleId)
                            .to(g_rules::Table, g_rules::Id),
                    )
                    .col(integer(g_audittrail::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_id")
                            .from(g_audittrail::Table, g_audittrail::UserId)
                            .to(g_appusers::Table, g_appusers::Id),
                    )
                    .col(date_time(g_audittrail::ChangesDoneAt).not_null().default("now()"))
                    .col(integer(g_audittrail::ResourceId))
                    .col(integer(g_audittrail::SubResourceId))
                    .col(json_binary(g_audittrail::ChangesJson).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(g_audittrail::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum g_audittrail {
    Table,
    Id,
    Action,
    Timestamp,
    WorkspaceId,
    ResourceId,
    UserId,
    RuleId,
    ChangesJson,
    ChangesDoneAt,
    SubResourceId,
    Details,
}

#[derive(DeriveIden)]
enum g_appusers {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum g_workspaces {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum g_rules{
    Table,
    Id,
}

use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000004_create_g_adittrail_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GAudittrail::Table)
                    .if_not_exists()
                    .col(pk_auto(GAudittrail::Id))
                    .col(string(GAudittrail::Action).not_null())
                    .col(integer(GAudittrail::WorkspaceId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("WorkspaceId")
                            .from(GAudittrail::Table, GAudittrail::WorkspaceId)
                            .to(g_workspaces::Table, g_workspaces::Id),                          
                    )
                    .col(timestamp_with_time_zone(GAudittrail::Timestamp).not_null().default(Expr::current_timestamp()))
                    .col(text(GAudittrail::Details).not_null())
                    .col(integer(GAudittrail::RuleId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rule_id")
                            .from(GAudittrail::Table, GAudittrail::RuleId)
                            .to(GRules::Table, GRules::Id),
                    )
                    .col(integer(GAudittrail::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_id")
                            .from(GAudittrail::Table, GAudittrail::UserId)
                            .to(GAppusers::Table, GAppusers::Id),
                    )
                    .col(date_time(GAudittrail::ChangesDoneAt).not_null().default("now()"))
                    .col(integer(GAudittrail::ResourceId))
                    .col(integer(GAudittrail::SubResourceId))
                    .col(json_binary(GAudittrail::ChangesJson).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GAudittrail::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GAudittrail {
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
enum GAppusers {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum g_workspaces {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum GRules{
    Table,
    Id,
}

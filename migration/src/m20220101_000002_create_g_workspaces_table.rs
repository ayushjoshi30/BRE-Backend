use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000002_create_g_workspaces_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(g_workspaces::Table)
                    .if_not_exists()
                    .col(pk_auto(g_workspaces::Id))
                    .col(string(g_workspaces::Identifier).not_null().unique_key())
                    .col(string(g_workspaces::OrganisationName).not_null().unique_key())
                    .col(text(g_workspaces::OrganisationAddress))
                    .col(string(g_workspaces::OrganisationEmail).not_null().unique_key())
                    .col(string(g_workspaces::AuthKey))
                    .col(string(g_workspaces::BaseUrl))
                    .col(text(g_workspaces::OrganizationLogo))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(g_workspaces::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum g_workspaces {
    Table,
    Id,
    Identifier,
    OrganisationName,
    OrganisationAddress,
    OrganisationEmail,
    AuthKey,
    BaseUrl,
    OrganizationLogo,
}

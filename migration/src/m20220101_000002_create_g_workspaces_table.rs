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
                    .table(GWorkspaces::Table)
                    .if_not_exists()
                    .col(pk_auto(GWorkspaces::Id))
                    .col(string(GWorkspaces::Identifier).not_null().unique_key())
                    .col(string(GWorkspaces::OrganisationName).not_null().unique_key())
                    .col(text(GWorkspaces::OrganisationAddress))
                    .col(string(GWorkspaces::OrganisationEmail).not_null().unique_key())
                    .col(string(GWorkspaces::AuthKey))
                    .col(string(GWorkspaces::BaseUrl))
                    .col(text(GWorkspaces::OrganizationLogo))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GWorkspaces::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum GWorkspaces {
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

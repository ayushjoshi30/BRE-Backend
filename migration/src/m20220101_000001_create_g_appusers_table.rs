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
        // Insert statement filling all the details
        

        // Create table statement
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
            .await?;
            let insert = Query::insert()
            .into_table(GAppusers::Table)
            .columns([
                GAppusers::FirstName,
                GAppusers::LastName,
                GAppusers::UserName,
                GAppusers::Email,
                GAppusers::MobileNo,
                GAppusers::CreatedOnDate,
                GAppusers::WorkspaceId,
                GAppusers::IsActive,
                GAppusers::IsDeleted,
                GAppusers::LastLogin,
                GAppusers::Password,
                GAppusers::IsAdmin,
            ])
            .values_panic([
                "System".into(),                          // FirstName
                "User".into(),                           // LastName
                "admin".into(),                         // UserName
                "admin@example.com".into(),             // Email
                "8003464814".into(),                    // MobileNo
                Expr::current_timestamp().into(),       // CreatedOnDate
                1.into(),                               // WorkspaceId
                true.into(),                            // IsActive
                false.into(),                           // IsDeleted
                Expr::current_timestamp().into(),       // LastLogin
                "9ee64312b6ef066abb2bb1cf5083d82b2ad945683c7051bb99c7845143334516".into(),               // Password
                true.into(),                            // IsAdmin
            ])
            .to_owned();

        manager.exec_stmt(insert).await?;
        Ok(())
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

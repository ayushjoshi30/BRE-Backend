pub use sea_orm_migration::prelude::*;

mod m20220101_000002_create_g_workspaces_table;
mod m20220101_000001_create_g_appusers_table;

mod m20220101_000001_create_g_rules_table;
mod m20220101_000001_create_g_releases_table;
mod m20220101_000003_create_g_configure_table;
mod m20220101_000001_create_g_audittrail_table;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000002_create_g_workspaces_table::Migration),
            Box::new(m20220101_000001_create_g_appusers_table::Migration),
            Box::new(m20220101_000001_create_g_rules_table::Migration),
            Box::new(m20220101_000001_create_g_releases_table::Migration),
            Box::new(m20220101_000003_create_g_configure_table::Migration),
            Box::new(m20220101_000001_create_g_audittrail_table::Migration)
        ]
    }
}

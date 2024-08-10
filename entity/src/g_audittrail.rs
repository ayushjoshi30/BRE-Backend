//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Serialize,Deserialize, Eq)]
#[sea_orm(table_name = "g_audittrail")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub workspace_id: i32,
    #[serde(default)]
    pub timestamp: DateTimeWithTimeZone,
    #[sea_orm(column_type = "Text")]
    #[serde(default)]
    pub details: String,
    #[serde(default)]
    pub rule_id: i32,
    #[serde(default)]
    pub user_id: i32,
    #[serde(default)]
    pub changes_done_at: DateTime,
    #[serde(default)]
    pub resource_id: i32,
    #[serde(default)]
    pub sub_resource_id: i32,
    #[sea_orm(column_type = "JsonBinary")]
    #[serde(default)]
    pub changes_json: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::g_appusers::Entity",
        from = "Column::UserId",
        to = "super::g_appusers::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    GAppusers,
    #[sea_orm(
        belongs_to = "super::g_rules::Entity",
        from = "Column::RuleId",
        to = "super::g_rules::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    GRules,
    #[sea_orm(
        belongs_to = "super::g_workspaces::Entity",
        from = "Column::WorkspaceId",
        to = "super::g_workspaces::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    GWorkspaces,
}

impl Related<super::g_appusers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GAppusers.def()
    }
}

impl Related<super::g_rules::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GRules.def()
    }
}

impl Related<super::g_workspaces::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GWorkspaces.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

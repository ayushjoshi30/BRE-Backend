//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Serialize,Deserialize, Eq)]
#[sea_orm(table_name = "g_rules")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[serde(default)]
    pub workspace_id: i32,
    pub rule_path: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub rule_json: Json,
    #[serde(default)]
    pub created_by_user: i32,
    #[serde(default = "current_time")]
    pub last_updated: DateTime,
    #[serde(default)]
    pub draft_file_path: String,
    #[sea_orm(column_type = "JsonBinary")]
    #[serde(default)]
    pub draft_file_json: Json,
    #[serde(default)]
    pub is_draft: bool,
    #[serde(default = "current_time")]
    pub published_at: DateTime,
    #[serde(default)]
    pub version: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::g_appusers::Entity",
        from = "Column::CreatedByUser",
        to = "super::g_appusers::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    GAppusers,
    #[sea_orm(has_many = "super::g_audittrail::Entity")]
    GAudittrail,
    #[sea_orm(
        belongs_to = "super::g_workspaces::Entity",
        from = "Column::WorkspaceId",
        to = "super::g_workspaces::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    GWorkspaces,
}
fn current_time() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}
impl Related<super::g_appusers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GAppusers.def()
    }
}

impl Related<super::g_audittrail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GAudittrail.def()
    }
}

impl Related<super::g_workspaces::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GWorkspaces.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
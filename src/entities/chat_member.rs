//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "chat_member")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_type = "Text")]
    pub chat_id: String,
    pub user_id: i32,
    pub cleared_history_at: Option<DateTime>,
    pub invited_by_user_id: i32,
    #[sea_orm(column_type = "Text")]
    pub invite_code: String,
    pub last_active_at: Option<DateTime>,
    pub unread_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
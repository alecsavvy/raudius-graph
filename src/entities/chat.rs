//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "chat")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_type = "Text")]
    pub chat_id: String,
    pub created_at: DateTime,
    pub last_message_at: DateTime,
    #[sea_orm(column_type = "Text", nullable)]
    pub last_message: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

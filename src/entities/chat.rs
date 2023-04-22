//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "chat")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub chat_id: String,
    pub created_at: DateTime,
    pub last_message_at: DateTime,
    #[sea_orm(column_type = "Text", nullable)]
    pub last_message: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::chat_member::Entity")]
    ChatMember,
}

impl Related<super::chat_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatMember.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
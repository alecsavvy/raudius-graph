//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use super::sea_orm_active_enums::Savetype;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "saves")]
pub struct Model {
    pub blockhash: Option<String>,
    pub blocknumber: Option<i32>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub save_item_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub save_type: Savetype,
    #[sea_orm(primary_key, auto_increment = false)]
    pub is_current: bool,
    pub is_delete: bool,
    pub created_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub txhash: String,
    pub slot: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::Blockhash",
        to = "super::blocks::Column::Blockhash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Blocks2,
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::Blocknumber",
        to = "super::blocks::Column::Number",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Blocks1,
}

impl ActiveModelBehavior for ActiveModel {}

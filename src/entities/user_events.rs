//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub blockhash: Option<String>,
    pub blocknumber: Option<i32>,
    pub is_current: bool,
    pub user_id: i32,
    pub referrer: Option<i32>,
    pub is_mobile_user: bool,
    pub slot: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "supporter_rank_ups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub slot: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub sender_user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub receiver_user_id: i32,
    pub rank: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

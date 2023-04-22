//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "aggregate_user_tips")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub sender_user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub receiver_user_id: i32,
    pub amount: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

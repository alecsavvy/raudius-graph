//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "hourly_play_counts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub hourly_timestamp: DateTime,
    pub play_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

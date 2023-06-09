//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "rpc_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub jetstream_sequence: i32,
    pub jetstream_timestamp: DateTime,
    #[sea_orm(column_type = "Text", nullable)]
    pub from_wallet: Option<String>,
    pub rpc: Json,
    #[sea_orm(column_type = "Text")]
    pub sig: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

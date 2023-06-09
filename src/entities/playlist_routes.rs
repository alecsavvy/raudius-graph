//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "playlist_routes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub slug: String,
    pub title_slug: String,
    pub collision_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub owner_id: i32,
    pub playlist_id: i32,
    pub is_current: bool,
    pub blockhash: String,
    pub blocknumber: i32,
    pub txhash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

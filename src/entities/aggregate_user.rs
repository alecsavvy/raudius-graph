//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "aggregate_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    pub track_count: Option<i64>,
    pub playlist_count: Option<i64>,
    pub album_count: Option<i64>,
    pub follower_count: Option<i64>,
    pub following_count: Option<i64>,
    pub repost_count: Option<i64>,
    pub track_save_count: Option<i64>,
    pub supporter_count: i32,
    pub supporting_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "trending_results")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub id: Option<String>,
    pub rank: i32,
    pub r#type: String,
    pub version: String,
    pub week: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

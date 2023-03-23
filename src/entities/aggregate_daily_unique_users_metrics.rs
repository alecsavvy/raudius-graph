//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "aggregate_daily_unique_users_metrics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub count: i32,
    pub timestamp: Date,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub summed_count: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

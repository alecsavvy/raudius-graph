//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "aggregate_monthly_app_name_metrics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub application_name: String,
    pub count: i32,
    pub timestamp: Date,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

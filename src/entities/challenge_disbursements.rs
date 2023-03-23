//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "challenge_disbursements")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub challenge_id: String,
    pub user_id: i32,
    pub specifier: String,
    pub signature: String,
    pub slot: i32,
    pub amount: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "supporter_rank_ups")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub slot: i32,
    pub sender_user_id: i32,
    pub receiver_user_id: i32,
    pub rank: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

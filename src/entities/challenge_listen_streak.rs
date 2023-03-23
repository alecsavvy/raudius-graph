//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "challenge_listen_streak")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub last_listen_date: Option<DateTime>,
    pub listen_streak: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_bank_accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub signature: String,
    pub ethereum_address: String,
    pub created_at: DateTime,
    pub bank_account: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

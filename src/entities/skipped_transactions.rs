use super::sea_orm_active_enums::Skippedtransactionlevel;
use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "skipped_transactions")]
#[graphql(complex)]
#[graphql(name = "SkippedTransactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub blocknumber: i32,
    pub blockhash: String,
    pub txhash: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub level: Skippedtransactionlevel,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

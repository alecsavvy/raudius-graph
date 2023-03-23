use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "user_bank_txs")]
#[graphql(complex)]
#[graphql(name = "UserBankTxs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub signature: String,
    pub slot: i32,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

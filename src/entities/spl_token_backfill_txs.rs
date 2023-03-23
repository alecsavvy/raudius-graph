use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "spl_token_backfill_txs")]
#[graphql(complex)]
#[graphql(name = "SplTokenBackfillTxs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub last_scanned_slot: i32,
    pub signature: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

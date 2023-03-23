use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "audio_transactions_history")]
#[graphql(complex)]
#[graphql(name = "AudioTransactionsHistory")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_bank: String,
    pub slot: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub signature: String,
    pub transaction_type: String,
    pub method: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub transaction_created_at: DateTime,
    pub change: Decimal,
    pub balance: Decimal,
    pub tx_metadata: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

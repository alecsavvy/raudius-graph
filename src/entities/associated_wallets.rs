use super::sea_orm_active_enums::WalletChain;
use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "associated_wallets")]
#[graphql(complex)]
#[graphql(name = "AssociatedWallets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub wallet: String,
    pub blockhash: String,
    pub blocknumber: i32,
    pub is_current: bool,
    pub is_delete: bool,
    pub chain: WalletChain,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

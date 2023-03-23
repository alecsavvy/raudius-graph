use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "user_balances")]
#[graphql(complex)]
#[graphql(name = "UserBalances")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub balance: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub associated_wallets_balance: String,
    pub waudio: Option<String>,
    pub associated_sol_wallets_balance: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

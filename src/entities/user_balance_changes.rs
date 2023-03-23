use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "user_balance_changes")]
#[graphql(complex)]
#[graphql(name = "UserBalanceChanges")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub blocknumber: i32,
    pub current_balance: String,
    pub previous_balance: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

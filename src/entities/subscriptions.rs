use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "subscriptions")]
#[graphql(complex)]
#[graphql(name = "Subscriptions")]
pub struct Model {
    pub blockhash: Option<String>,
    pub blocknumber: Option<i32>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub subscriber_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub is_current: bool,
    pub is_delete: bool,
    pub created_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub txhash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "aggregate_user_tips")]
#[graphql(complex)]
#[graphql(name = "AggregateUserTips")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub sender_user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub receiver_user_id: i32,
    pub amount: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

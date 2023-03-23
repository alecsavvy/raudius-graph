use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "supporter_rank_ups")]
#[graphql(complex)]
#[graphql(name = "SupporterRankUps")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub slot: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub sender_user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub receiver_user_id: i32,
    pub rank: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

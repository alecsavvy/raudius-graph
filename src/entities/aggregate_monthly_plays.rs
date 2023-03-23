use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "aggregate_monthly_plays")]
#[graphql(complex)]
#[graphql(name = "AggregateMonthlyPlays")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub play_item_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub timestamp: Date,
    pub count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

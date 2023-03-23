use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "aggregate_track")]
#[graphql(complex)]
#[graphql(name = "AggregateTrack")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub track_id: i32,
    pub repost_count: i32,
    pub save_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

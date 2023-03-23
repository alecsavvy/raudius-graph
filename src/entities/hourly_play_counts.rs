use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "hourly_play_counts")]
#[graphql(complex)]
#[graphql(name = "HourlyPlayCounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub hourly_timestamp: DateTime,
    pub play_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

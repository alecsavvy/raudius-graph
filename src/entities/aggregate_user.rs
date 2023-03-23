use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "aggregate_user")]
#[graphql(complex)]
#[graphql(name = "AggregateUser")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    pub track_count: Option<i64>,
    pub playlist_count: Option<i64>,
    pub album_count: Option<i64>,
    pub follower_count: Option<i64>,
    pub following_count: Option<i64>,
    pub repost_count: Option<i64>,
    pub track_save_count: Option<i64>,
    pub supporter_count: i32,
    pub supporting_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

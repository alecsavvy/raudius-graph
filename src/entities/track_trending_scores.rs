use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "track_trending_scores")]
#[graphql(complex)]
#[graphql(name = "TrackTrendingScores")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub track_id: i32,
    #[sea_orm(primary_key, auto_increment = false, column_name = "type")]
    pub model_type: String,
    pub genre: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub version: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub time_range: String,
    pub score: f64,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

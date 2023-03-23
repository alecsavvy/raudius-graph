use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "trending_results")]
#[graphql(complex)]
#[graphql(name = "TrendingResults")]
pub struct Model {
    pub user_id: i32,
    pub id: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub rank: i32,
    #[sea_orm(primary_key, auto_increment = false, column_name = "type")]
    pub model_type: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub version: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub week: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

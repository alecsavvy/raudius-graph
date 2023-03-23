use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "aggregate_daily_app_name_metrics")]
#[graphql(complex)]
#[graphql(name = "AggregateDailyAppNameMetrics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub application_name: String,
    pub count: i32,
    pub timestamp: Date,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "aggregate_plays")]
#[graphql(complex)]
#[graphql(name = "AggregatePlays")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub play_item_id: i32,
    pub count: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

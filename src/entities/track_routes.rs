use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "track_routes")]
#[graphql(complex)]
#[graphql(name = "TrackRoutes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub slug: String,
    pub title_slug: String,
    pub collision_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub owner_id: i32,
    pub track_id: i32,
    pub is_current: bool,
    pub blockhash: String,
    pub blocknumber: i32,
    pub txhash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

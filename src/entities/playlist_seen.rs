use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "playlist_seen")]
#[graphql(complex)]
#[graphql(name = "PlaylistSeen")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub playlist_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub seen_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub is_current: bool,
    pub blocknumber: Option<i32>,
    pub blockhash: Option<String>,
    pub txhash: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "plays_archive")]
#[graphql(complex)]
#[graphql(name = "PlaysArchive")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub user_id: Option<i32>,
    pub source: Option<String>,
    pub play_item_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub slot: Option<i32>,
    pub signature: Option<String>,
    pub archived_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

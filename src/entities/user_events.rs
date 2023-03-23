use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "user_events")]
#[graphql(complex)]
#[graphql(name = "UserEvents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub blockhash: Option<String>,
    pub blocknumber: Option<i32>,
    pub is_current: bool,
    pub user_id: i32,
    pub referrer: Option<i32>,
    pub is_mobile_user: bool,
    pub slot: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

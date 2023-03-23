use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "blocks_copy")]
#[graphql(complex)]
#[graphql(name = "BlocksCopy")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub blockhash: String,
    pub parenthash: Option<String>,
    pub is_current: Option<bool>,
    #[sea_orm(unique)]
    pub number: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

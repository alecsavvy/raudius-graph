use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "cid_data")]
#[graphql(complex)]
#[graphql(name = "CidData")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub cid: String,
    #[sea_orm(column_name = "type")]
    pub model_type: Option<String>,
    // pub data: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

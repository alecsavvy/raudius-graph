use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "notification")]
#[graphql(complex)]
#[graphql(name = "Notification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub specifier: String,
    pub group_id: String,
    #[sea_orm(column_name = "type")]
    pub model_type: String,
    pub slot: Option<i32>,
    pub blocknumber: Option<i32>,
    pub timestamp: DateTime,
    // pub data: Option<Json>,
    #[sea_orm(column_type = "Custom(\"array\".to_owned())", nullable)]
    pub user_ids: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

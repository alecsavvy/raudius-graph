use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "reactions")]
#[graphql(complex)]
#[graphql(name = "Reactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub slot: i32,
    pub reaction_value: i32,
    pub sender_wallet: String,
    pub reaction_type: String,
    pub reacted_to: String,
    pub timestamp: DateTime,
    pub tx_signature: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

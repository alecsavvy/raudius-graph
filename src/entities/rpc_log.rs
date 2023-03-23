use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "rpc_log")]
#[graphql(complex)]
#[graphql(name = "RpcLog")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub jetstream_sequence: i32,
    pub jetstream_timestamp: DateTime,
    #[sea_orm(column_type = "Text", nullable)]
    pub from_wallet: Option<String>,
    // pub rpc: Json,
    #[sea_orm(column_type = "Text")]
    pub sig: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

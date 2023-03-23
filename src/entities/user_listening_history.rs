use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "user_listening_history")]
#[graphql(complex)]
#[graphql(name = "UserListeningHistory")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    // pub listening_history: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "chat_blocked_users")]
#[graphql(complex)]
#[graphql(name = "ChatBlockedUsers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub blocker_user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub blockee_user_id: i32,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

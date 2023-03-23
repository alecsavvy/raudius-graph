use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "user_tips")]
#[graphql(complex)]
#[graphql(name = "UserTips")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub slot: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub signature: String,
    pub sender_user_id: i32,
    pub receiver_user_id: i32,
    pub amount: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

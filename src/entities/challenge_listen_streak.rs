use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "challenge_listen_streak")]
#[graphql(complex)]
#[graphql(name = "ChallengeListenStreak")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub last_listen_date: Option<DateTime>,
    pub listen_streak: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "user_challenges")]
#[graphql(complex)]
#[graphql(name = "UserChallenges")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub challenge_id: String,
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub specifier: String,
    pub is_complete: bool,
    pub current_step_count: Option<i32>,
    pub completed_blocknumber: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::challenges::Entity",
        from = "Column::ChallengeId",
        to = "super::challenges::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Challenges,
}

impl Related<super::challenges::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Challenges.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "challenge_profile_completion")]
#[graphql(complex)]
#[graphql(name = "ChallengeProfileCompletion")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub profile_description: bool,
    pub profile_name: bool,
    pub profile_picture: bool,
    pub profile_cover_photo: bool,
    pub follows: bool,
    pub favorites: bool,
    pub reposts: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

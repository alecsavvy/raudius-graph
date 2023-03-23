use super::sea_orm_active_enums::Challengetype;
use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "challenges")]
#[graphql(complex)]
#[graphql(name = "Challenges")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "type")]
    pub model_type: Challengetype,
    pub amount: String,
    pub active: bool,
    pub step_count: Option<i32>,
    pub starting_block: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_challenges::Entity")]
    UserChallenges,
}

impl Related<super::user_challenges::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserChallenges.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

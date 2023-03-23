use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "challenge_disbursements")]
#[graphql(complex)]
#[graphql(name = "ChallengeDisbursements")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub challenge_id: String,
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub specifier: String,
    pub signature: String,
    pub slot: i32,
    pub amount: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

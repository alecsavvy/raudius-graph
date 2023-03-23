use super::sea_orm_active_enums::Savetype;
use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "saves")]
#[graphql(complex)]
#[graphql(name = "Saves")]
pub struct Model {
    pub blockhash: Option<String>,
    pub blocknumber: Option<i32>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub save_item_id: i32,
    //#[sea_orm(primary_key, auto_increment = false)]
    pub save_type: Savetype,
    #[sea_orm(primary_key, auto_increment = false)]
    pub is_current: bool,
    pub is_delete: bool,
    pub created_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub txhash: String,
    pub slot: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::Blockhash",
        to = "super::blocks::Column::Blockhash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Blocks2,
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::Blocknumber",
        to = "super::blocks::Column::Number",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Blocks1,
}

impl ActiveModelBehavior for ActiveModel {}

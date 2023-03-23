use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "ursm_content_nodes")]
#[graphql(complex)]
#[graphql(name = "UrsmContentNodes")]
pub struct Model {
    pub blockhash: Option<String>,
    pub blocknumber: Option<i32>,
    pub created_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub is_current: bool,
    #[sea_orm(primary_key, auto_increment = false)]
    pub cnode_sp_id: i32,
    pub delegate_owner_wallet: String,
    pub owner_wallet: String,
    #[sea_orm(column_type = "Custom(\"array\".to_owned())")]
    pub proposer_sp_ids: String,
    pub proposer_1_delegate_owner_wallet: String,
    pub proposer_2_delegate_owner_wallet: String,
    pub proposer_3_delegate_owner_wallet: String,
    pub endpoint: Option<String>,
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

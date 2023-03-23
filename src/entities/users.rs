use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "users")]
#[graphql(complex)]
#[graphql(name = "Users")]
pub struct Model {
    pub blockhash: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub is_current: bool,
    pub handle: Option<String>,
    pub wallet: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: Option<String>,
    pub profile_picture: Option<String>,
    pub cover_photo: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub metadata_multihash: Option<String>,
    pub creator_node_endpoint: Option<String>,
    pub blocknumber: Option<i32>,
    pub is_verified: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub handle_lc: Option<String>,
    pub cover_photo_sizes: Option<String>,
    pub profile_picture_sizes: Option<String>,
    pub primary_id: Option<i32>,
    //#[sea_orm(column_type = "Custom(\"array\".to_owned())", nullable)]
    //pub secondary_ids: Option<String>,
    pub replica_set_update_signer: Option<String>,
    pub has_collectibles: bool,
    #[sea_orm(primary_key, auto_increment = false)]
    pub txhash: String,
    // pub playlist_library: Option<Json>,
    pub is_deactivated: bool,
    pub slot: Option<i32>,
    pub user_storage_account: Option<String>,
    pub user_authority_account: Option<String>,
    pub artist_pick_track_id: Option<i32>,
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

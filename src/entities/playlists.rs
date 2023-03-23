use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "playlists")]
#[graphql(complex)]
#[graphql(name = "Playlists")]
pub struct Model {
    pub blockhash: Option<String>,
    pub blocknumber: Option<i32>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub playlist_id: i32,
    pub playlist_owner_id: i32,
    pub is_album: bool,
    pub is_private: bool,
    pub playlist_name: Option<String>,
    // pub playlist_contents: Json,
    pub playlist_image_multihash: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub is_current: bool,
    pub is_delete: bool,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub upc: Option<String>,
    pub updated_at: DateTime,
    pub playlist_image_sizes_multihash: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub txhash: String,
    pub last_added_to: Option<DateTime>,
    pub slot: Option<i32>,
    pub metadata_multihash: Option<String>,
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

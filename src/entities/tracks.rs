use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "tracks")]
#[graphql(complex)]
#[graphql(name = "Tracks")]
pub struct Model {
    pub blockhash: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub track_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub is_current: bool,
    pub is_delete: bool,
    pub owner_id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub title: Option<String>,
    pub length: Option<i32>,
    pub cover_art: Option<String>,
    pub tags: Option<String>,
    pub genre: Option<String>,
    pub mood: Option<String>,
    pub credits_splits: Option<String>,
    pub create_date: Option<String>,
    pub release_date: Option<String>,
    pub file_type: Option<String>,
    pub metadata_multihash: Option<String>,
    pub blocknumber: Option<i32>,
    // pub track_segments: Json,
    pub created_at: DateTime,
    pub description: Option<String>,
    pub isrc: Option<String>,
    pub iswc: Option<String>,
    pub license: Option<String>,
    pub updated_at: DateTime,
    pub cover_art_sizes: Option<String>,
    // pub download: Option<Json>,
    pub is_unlisted: bool,
    // pub field_visibility: Option<Json>,
    pub route_id: Option<String>,
    // pub stem_of: Option<Json>,
    // pub remix_of: Option<Json>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub txhash: String,
    pub slot: Option<i32>,
    pub is_available: bool,
    pub is_premium: bool,
    // pub premium_conditions: Option<Json>,
    pub track_cid: Option<String>,
    pub is_playlist_upload: bool,
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

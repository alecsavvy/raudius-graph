//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tracks")]
pub struct Model {
    pub blockhash: Option<String>,
    #[sea_orm(primary_key)]
    pub track_id: i32,
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
    #[sea_orm(column_type = "JsonBinary")]
    pub track_segments: Json,
    pub created_at: DateTime,
    pub description: Option<String>,
    pub isrc: Option<String>,
    pub iswc: Option<String>,
    pub license: Option<String>,
    pub updated_at: DateTime,
    pub cover_art_sizes: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub download: Option<Json>,
    pub is_unlisted: bool,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub field_visibility: Option<Json>,
    pub route_id: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub stem_of: Option<Json>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub remix_of: Option<Json>,
    pub txhash: String,
    pub slot: Option<i32>,
    pub is_available: bool,
    pub is_premium: bool,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub premium_conditions: Option<Json>,
    pub track_cid: Option<String>,
    pub is_playlist_upload: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

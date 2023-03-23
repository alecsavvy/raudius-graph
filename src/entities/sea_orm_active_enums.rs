use sea_orm::entity::prelude::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    EnumIter,
    DeriveActiveEnum,
    Eq,
    Copy,
    async_graphql::Enum,
    seaography::macros::EnumFilter,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "skippedtransactionlevel"
)]
pub enum Skippedtransactionlevel {
    #[sea_orm(string_value = "network")]
    Network,
    #[sea_orm(string_value = "node")]
    Node,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    EnumIter,
    DeriveActiveEnum,
    Eq,
    Copy,
    async_graphql::Enum,
    seaography::macros::EnumFilter,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "wallet_chain")]
pub enum WalletChain {
    #[sea_orm(string_value = "eth")]
    Eth,
    #[sea_orm(string_value = "sol")]
    Sol,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    EnumIter,
    DeriveActiveEnum,
    Eq,
    Copy,
    async_graphql::Enum,
    seaography::macros::EnumFilter,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "reposttype")]
pub enum Reposttype {
    #[sea_orm(string_value = "album")]
    Album,
    #[sea_orm(string_value = "playlist")]
    Playlist,
    #[sea_orm(string_value = "track")]
    Track,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    EnumIter,
    DeriveActiveEnum,
    Eq,
    Copy,
    async_graphql::Enum,
    seaography::macros::EnumFilter,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "savetype")]
pub enum Savetype {
    #[sea_orm(string_value = "album")]
    Album,
    #[sea_orm(string_value = "playlist")]
    Playlist,
    #[sea_orm(string_value = "track")]
    Track,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    EnumIter,
    DeriveActiveEnum,
    Eq,
    Copy,
    async_graphql::Enum,
    seaography::macros::EnumFilter,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "challengetype")]
pub enum Challengetype {
    #[sea_orm(string_value = "aggregate")]
    Aggregate,
    #[sea_orm(string_value = "boolean")]
    Boolean,
    #[sea_orm(string_value = "numeric")]
    Numeric,
    #[sea_orm(string_value = "trending")]
    Trending,
}

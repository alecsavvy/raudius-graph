use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "chat")]
#[graphql(complex)]
#[graphql(name = "Chat")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub chat_id: String,
    pub created_at: DateTime,
    pub last_message_at: DateTime,
    #[sea_orm(column_type = "Text", nullable)]
    pub last_message: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::chat_member::Entity")]
    ChatMember,
}

impl Related<super::chat_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatMember.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

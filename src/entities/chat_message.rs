use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "chat_message")]
#[graphql(complex)]
#[graphql(name = "ChatMessage")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub message_id: String,
    #[sea_orm(column_type = "Text")]
    pub chat_id: String,
    pub user_id: i32,
    pub created_at: DateTime,
    #[sea_orm(column_type = "Text")]
    pub ciphertext: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::chat_member::Entity",
        from = "Column::ChatId",
        to = "super::chat_member::Column::ChatId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ChatMember,
    #[sea_orm(has_many = "super::chat_message_reactions::Entity")]
    ChatMessageReactions,
}

impl Related<super::chat_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatMember.def()
    }
}

impl Related<super::chat_message_reactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatMessageReactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

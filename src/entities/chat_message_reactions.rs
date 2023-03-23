use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "chat_message_reactions")]
#[graphql(complex)]
#[graphql(name = "ChatMessageReactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub message_id: String,
    #[sea_orm(column_type = "Text")]
    pub reaction: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::chat_message::Entity",
        from = "Column::MessageId",
        to = "super::chat_message::Column::MessageId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ChatMessage,
}

impl Related<super::chat_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatMessage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

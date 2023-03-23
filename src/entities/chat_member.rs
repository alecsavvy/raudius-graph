use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "chat_member")]
#[graphql(complex)]
#[graphql(name = "ChatMember")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub chat_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    pub cleared_history_at: Option<DateTime>,
    pub invited_by_user_id: i32,
    #[sea_orm(column_type = "Text")]
    pub invite_code: String,
    pub last_active_at: Option<DateTime>,
    pub unread_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::chat::Entity",
        from = "Column::ChatId",
        to = "super::chat::Column::ChatId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Chat,
    #[sea_orm(has_many = "super::chat_message::Entity")]
    ChatMessage,
}

impl Related<super::chat::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chat.def()
    }
}

impl Related<super::chat_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatMessage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

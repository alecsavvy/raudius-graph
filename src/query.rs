use crate::entities::{prelude::*, *};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

pub type AudiusSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn howdy(&self) -> &'static str {
        "partner"
    }

    /* User Queries */
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<users::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Users::find().all(db).await
    }

    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<Option<users::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Users::find_by_id(id).one(db).await
    }
}

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::entities::{prelude::*, *};

macro_rules! q {
    ($([$mod_name:ident, $entity:ty]),+) => {
        pub struct QueryRoot;

        #[Object]
        impl QueryRoot {
            async fn howdy(&self) -> &'static str {
                "partner 🤠"
            }

            $(
                async fn $mod_name(&self, ctx: &Context<'_>) -> Result<Vec<$mod_name::Model>, DbErr> {
                    let db = ctx
                        .data::<DatabaseConnection>()
                        .map_err(|e| DbErr::Custom(e.message))?;
                    <$entity>::find().all(db).await
                }
            ),+

            async fn user(
                &self,
                ctx: &Context<'_>,
                id: i32,
            ) -> Result<Option<users::Model>, DbErr> {
                let db = ctx
                    .data::<DatabaseConnection>()
                    .map_err(|e| DbErr::Custom(e.message))?;
                Users::find_by_id(id).one(db).await
            }
        }
    };
}

q!([users, Users]);

pub type AudiusSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

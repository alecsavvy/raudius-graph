use crate::entities::users::{self, Entity as User};
use async_graphql::Object;

#[derive(Debug, Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, limit: Option<usize>) -> Vec<String> {
        let limit = limit.unwrap_or(100);
        let users = User::find()
            .cursor_by((users::Column::UserId, users::Column::IsCurrent))
            .first(limit)
            .all()
            .await
            .unwrap();
        vec![]
    }
}

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub type AudiusSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn howdy(&self) -> &'static str {
        "partner"
    }
}

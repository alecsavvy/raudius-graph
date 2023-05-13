pub mod playlists;
pub mod tips;
pub mod tracks;
pub mod users;

use async_graphql::MergedObject;
pub use users::*;

#[derive(Debug, MergedObject, Default)]
pub struct RootQuery(UserQuery);

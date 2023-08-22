use async_graphql::MergedObject;

pub mod get_user;
pub mod login;

use get_user::UserQuery;

#[derive(MergedObject, Default)]
pub struct UsersQuery(pub UserQuery);
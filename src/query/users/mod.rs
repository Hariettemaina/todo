use async_graphql::MergedObject;
pub mod get_user;
pub mod login;

use self::{login::LoginQuery, get_user::UserQuery};

#[derive(MergedObject, Default)]
pub struct Query(
    pub LoginQuery,
    pub UserQuery
);

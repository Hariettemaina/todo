use async_graphql::MergedObject;

use self::{login::LoginQuery, get_user::UserQuery};

pub mod get_user;
pub mod login;
pub mod get_todo;
pub mod get_category;

#[derive(MergedObject, Default)]
pub struct Query(
    pub LoginQuery,
    pub UserQuery
);

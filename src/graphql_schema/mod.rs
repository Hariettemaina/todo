use async_graphql::SimpleObject;

pub mod category;
pub mod category_lookup;
pub mod todo;
pub mod users;

use crate::graphql_schema::category::query::CategorysQuery;
use crate::graphql_schema::todo::query::TodosQuery;
use crate::graphql_schema::users::query::UsersQuery;
//use crate::graphql_schema::users::query::UsersQuery;

#[derive(SimpleObject, Default)]
pub struct Query {
    pub users: UsersQuery,
    pub todo: TodosQuery,
    pub category: CategorysQuery,
}

use crate::graphql_schema::category::mutation::AddCategoryMutation;
use crate::graphql_schema::category_lookup::mutation::CategorysMutation;
use crate::graphql_schema::todo::mutation::TodoMutation;
use crate::graphql_schema::users::mutation::UserMutation;

#[derive(SimpleObject, Default)]
pub struct Mutation {
    pub user: UserMutation,
    pub todo: TodoMutation,
    pub category: AddCategoryMutation,
    pub category_lookup: CategorysMutation,
}
